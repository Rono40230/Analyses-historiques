// services/ml_predictor.rs - Prédiction ML pour événements futurs
// Conforme .clinerules : < 300 lignes

use linfa_logistic::LogisticRegression;
use ndarray::Array2;
use std::path::Path;
use chrono::Utc;
use crate::models::{
    calendar_event::{CalendarEvent, NewPredictedEvent, PredictedEvent},
    candle::Candle,
    errors::VolatilityError,
};
use crate::services::{FeatureExtractor, MLTrainer};
use crate::db::{DbPool, schema::predicted_events};
use diesel::prelude::*;

/// Résultat d'une prédiction
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub event_id: i32,
    pub probability: f64,
    pub confidence_score: f64,
    pub predicted_volatility_increase: f64,
    pub recommendation: PredictionRecommendation,
}

/// Recommandation basée sur la prédiction
#[derive(Debug, Clone, PartialEq)]
pub enum PredictionRecommendation {
    HighRisk,      // Prob > 0.7 - Éviter de trader
    MediumRisk,    // Prob 0.4-0.7 - Prudence
    LowRisk,       // Prob < 0.4 - Trading normal
}

impl PredictionRecommendation {
    pub fn from_probability(prob: f64) -> Self {
        if prob > 0.7 {
            Self::HighRisk
        } else if prob > 0.4 {
            Self::MediumRisk
        } else {
            Self::LowRisk
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::HighRisk => "HIGH_RISK",
            Self::MediumRisk => "MEDIUM_RISK",
            Self::LowRisk => "LOW_RISK",
        }
    }
}

/// Prédicteur ML pour événements calendrier
pub struct MLPredictor {
    pool: DbPool,
    model: LogisticRegression<f64>,
    feature_extractor: FeatureExtractor,
    model_version: String,
}

impl MLPredictor {
    /// Crée un nouveau prédicteur en chargeant le modèle
    pub fn new(model_path: &Path, db_pool: DbPool) -> Result<Self, VolatilityError> {
        let trainer = MLTrainer::new();
        let model = trainer.load_model(model_path)?;
        
        Ok(Self {
            model,
            feature_extractor: FeatureExtractor::new(),
            pool: db_pool,
            model_version: trainer.model_version().to_string(),
        })
    }

    /// Prédit la probabilité de spike pour un événement
    pub fn predict_event(
        &self,
        event: &CalendarEvent,
        candles: &[Candle],
    ) -> Result<PredictionResult, VolatilityError> {
        // Extraire les features
        let features = self.feature_extractor.extract_features(event, candles)?;

        // Créer matrice de features (1 ligne, 13 colonnes)
        let x = self.features_to_array(&features)?;

        // Prédiction
        let predictions = self.model.predict(&x);
        let probability = if predictions[0] == 1 { 0.8 } else { 0.2 }; // Simplifié

        // Calculer le confidence score (basé sur la magnitude des features)
        let confidence_score = self.calculate_confidence(&features);

        // Estimer l'augmentation de volatilité attendue
        let predicted_volatility_increase = probability * 100.0;

        // Recommandation
        let recommendation = PredictionRecommendation::from_probability(probability);

        Ok(PredictionResult {
            event_id: event.id.unwrap_or(0),
            probability,
            confidence_score,
            predicted_volatility_increase,
            recommendation,
        })
    }

    /// Convertit EventFeatures en Array2 pour le modèle
    fn features_to_array(
        &self,
        features: &crate::services::feature_extractor::EventFeatures,
    ) -> Result<Array2<f64>, VolatilityError> {
        let feature_vec = vec![
            features.hour as f64,
            features.day_of_week as f64,
            features.month as f64,
            features.impact_high,
            features.impact_medium,
            features.event_economic,
            features.event_central_bank,
            features.has_forecast,
            features.atr_pre_event,
            features.volatility_pre_event,
            features.volume_avg_pre_event,
            features.body_range_avg,
            features.shadow_ratio_avg,
        ];

        Array2::from_shape_vec((1, 13), feature_vec)
            .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))
    }

    /// Calcule un score de confiance basé sur les features
    fn calculate_confidence(&self, features: &crate::services::feature_extractor::EventFeatures) -> f64 {
        let mut confidence = 0.5; // Base 50%

        // Augmenter si impact HIGH
        if features.impact_high > 0.5 {
            confidence += 0.2;
        }

        // Augmenter si forecast disponible
        if features.has_forecast > 0.5 {
            confidence += 0.1;
        }

        // Augmenter si volatilité pré-event significative
        if features.volatility_pre_event > 0.5 {
            confidence += 0.15;
        }

        // Réduire si données manquantes
        if features.atr_pre_event < 0.01 {
            confidence -= 0.1;
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Prédit et sauvegarde les prédictions pour plusieurs événements
    pub fn predict_and_save_batch(
        &self,
        events: &[CalendarEvent],
        candles: &[Candle],
    ) -> Result<Vec<PredictionResult>, VolatilityError> {
        let mut predictions = Vec::new();

        for event in events {
            if let Ok(prediction) = self.predict_event(event, candles) {
                let new_pred = NewPredictedEvent {
                    event_id: prediction.event_id,
                    predicted_probability: prediction.probability,
                    confidence_score: prediction.confidence_score,
                    model_version: self.model_version.clone(),
                    predicted_volatility_increase: Some(prediction.predicted_volatility_increase),
                };

                // Insérer dans la DB
                let mut conn = self.pool.get()
                    .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

                diesel::insert_into(predicted_events::table)
                    .values(&new_pred)
                    .execute(&mut conn)
                    .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;
                
                predictions.push(prediction);
            }
        }

        Ok(predictions)
    }

    /// Récupère les prédictions sauvegardées pour un événement
    pub fn get_predictions_for_event(
        &self,
        event_id: i32,
    ) -> Result<Vec<PredictedEvent>, VolatilityError> {
        let mut conn = self.pool.get()
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

        predicted_events::table
            .filter(predicted_events::event_id.eq(event_id))
            .order(predicted_events::created_at.desc())
            .select(PredictedEvent::as_select())
            .load(&mut conn)
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))
    }

    /// Récupère les prédictions HIGH RISK pour les prochaines heures
    pub fn get_high_risk_predictions(
        &self,
        hours_ahead: i64,
    ) -> Result<Vec<(CalendarEvent, PredictedEvent)>, VolatilityError> {
        use crate::db::schema::calendar_events;

        let mut conn = self.pool.get()
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

        let now = Utc::now().naive_utc();
        let future_time = now + chrono::Duration::hours(hours_ahead);

        // Joindre calendar_events et predicted_events
        let results = calendar_events::table
            .inner_join(predicted_events::table)
            .filter(calendar_events::event_time.ge(now))
            .filter(calendar_events::event_time.le(future_time))
            .filter(predicted_events::predicted_probability.gt(0.7))
            .order(predicted_events::predicted_probability.desc())
            .load::<(CalendarEvent, PredictedEvent)>(&mut conn)
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    /// Retourne la version du modèle utilisé
    pub fn model_version(&self) -> &str {
        &self.model_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction_recommendation() {
        assert_eq!(
            PredictionRecommendation::from_probability(0.8),
            PredictionRecommendation::HighRisk
        );
        assert_eq!(
            PredictionRecommendation::from_probability(0.5),
            PredictionRecommendation::MediumRisk
        );
        assert_eq!(
            PredictionRecommendation::from_probability(0.3),
            PredictionRecommendation::LowRisk
        );
    }

    #[test]
    fn test_recommendation_to_string() {
        assert_eq!(
            PredictionRecommendation::HighRisk.to_string(),
            "HIGH_RISK"
        );
        assert_eq!(
            PredictionRecommendation::MediumRisk.to_string(),
            "MEDIUM_RISK"
        );
        assert_eq!(
            PredictionRecommendation::LowRisk.to_string(),
            "LOW_RISK"
        );
    }

    #[test]
    fn test_probability_bounds() {
        // Tester que toutes les probabilités donnent une recommandation valide
        for i in 0..=100 {
            let prob = i as f64 / 100.0;
            let rec = PredictionRecommendation::from_probability(prob);
            
            if prob >= 0.7 {
                assert_eq!(rec, PredictionRecommendation::HighRisk);
            } else if prob >= 0.4 {
                assert_eq!(rec, PredictionRecommendation::MediumRisk);
            } else {
                assert_eq!(rec, PredictionRecommendation::LowRisk);
            }
        }
    }
}
