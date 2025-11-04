// services/ml_trainer.rs - Entraînement modèle ML avec Linfa
// Conforme .clinerules : < 300 lignes

use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{Array1, Array2};
use polars::prelude::*;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use crate::models::errors::VolatilityError;

/// Résultats de l'entraînement
#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub train_samples: usize,
    pub test_samples: usize,
}

/// Service d'entraînement ML
pub struct MLTrainer {
    model_version: String,
}

impl MLTrainer {
    /// Crée un nouveau trainer
    pub fn new() -> Self {
        Self {
            model_version: "v1.0.0".to_string(),
        }
    }

    /// Entraîne un modèle de régression logistique
    pub fn train(
        &self,
        df: &DataFrame,
    ) -> Result<(LogisticRegression<f64>, TrainingMetrics), VolatilityError> {
        // Séparer features et target
        let (x_train, y_train, x_test, y_test) = self.split_train_test(df, 0.8)?;

        // Entraîner le modèle
        let model = LogisticRegression::default()
            .max_iterations(300)
            .gradient_tolerance(1e-5)
            .fit(&Dataset::new(x_train.clone(), y_train.clone()))
            .map_err(|e| VolatilityError::DatabaseError(format!("Erreur entraînement: {}", e)))?;

        // Calculer les métriques
        let metrics = self.calculate_metrics(&model, &x_test, &y_test)?;

        Ok((model, metrics))
    }

    /// Split train/test avec ratio configurable
    fn split_train_test(
        &self,
        df: &DataFrame,
        train_ratio: f64,
    ) -> Result<(Array2<f64>, Array1<usize>, Array2<f64>, Array1<usize>), VolatilityError> {
        let n_rows = df.height();
        let n_train = (n_rows as f64 * train_ratio) as usize;

        // Colonnes features (toutes sauf "target")
        let feature_cols = vec![
            "hour", "day_of_week", "month",
            "impact_high", "impact_medium",
            "event_economic", "event_central_bank",
            "has_forecast",
            "atr_pre_event", "volatility_pre_event",
            "volume_avg_pre_event", "body_range_avg",
            "shadow_ratio_avg",
        ];

        // Extraire features
        let mut x_data = Vec::new();
        for col_name in &feature_cols {
            let col = df.column(col_name)
                .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;
            
            let values = self.column_to_f64_vec(col)?;
            x_data.push(values);
        }

        // Transpose pour avoir shape (n_samples, n_features)
        let n_features = x_data.len();
        let mut x_matrix = vec![0.0; n_rows * n_features];
        for (i, feature_vec) in x_data.iter().enumerate() {
            for (j, &val) in feature_vec.iter().enumerate() {
                x_matrix[j * n_features + i] = val;
            }
        }

        // Extraire target
        let target_col = df.column("target")
            .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;
        let y_data = self.column_to_usize_vec(target_col)?;

        // Split
        let x_train_data = x_matrix[0..n_train * n_features].to_vec();
        let x_test_data = x_matrix[n_train * n_features..].to_vec();
        let y_train_data = y_data[0..n_train].to_vec();
        let y_test_data = y_data[n_train..].to_vec();

        let x_train = Array2::from_shape_vec((n_train, n_features), x_train_data)
            .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;
        
        let x_test = Array2::from_shape_vec((n_rows - n_train, n_features), x_test_data)
            .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;
        
        let y_train = Array1::from_vec(y_train_data);
        let y_test = Array1::from_vec(y_test_data);

        Ok((x_train, y_train, x_test, y_test))
    }

    /// Convertit une colonne Polars en Vec<f64>
    fn column_to_f64_vec(&self, col: &Series) -> Result<Vec<f64>, VolatilityError> {
        // Essayer d'abord comme f64
        if let Ok(float_series) = col.f64() {
            return Ok(float_series.into_iter()
                .map(|v| v.unwrap_or(0.0))
                .collect());
        }
        
        // Sinon essayer u8 (pour hour, day_of_week, month)
        if let Ok(u8_series) = col.u8() {
            return Ok(u8_series.into_iter()
                .map(|v| v.unwrap_or(0) as f64)
                .collect());
        }

        Err(VolatilityError::InvalidCsvData(
            format!("Impossible de convertir la colonne {} en f64", col.name())
        ))
    }

    /// Convertit target en Vec<usize> (0 ou 1)
    fn column_to_usize_vec(&self, col: &Series) -> Result<Vec<usize>, VolatilityError> {
        let float_series = col.f64()
            .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;
        
        Ok(float_series.into_iter()
            .map(|v| v.unwrap_or(0.0) as usize)
            .collect())
    }

    /// Calcule les métriques de performance
    fn calculate_metrics(
        &self,
        model: &LogisticRegression<f64>,
        x_test: &Array2<f64>,
        y_test: &Array1<usize>,
    ) -> Result<TrainingMetrics, VolatilityError> {
        // Prédictions
        let predictions = model.predict(x_test);

        let mut tp = 0; // True Positives
        let mut fp = 0; // False Positives
        let mut tn = 0; // True Negatives
        let mut fn_count = 0; // False Negatives

        for (pred, &actual) in predictions.iter().zip(y_test.iter()) {
            match (*pred, actual) {
                (1, 1) => tp += 1,
                (1, 0) => fp += 1,
                (0, 0) => tn += 1,
                (0, 1) => fn_count += 1,
                _ => {}
            }
        }

        let total = y_test.len() as f64;
        let accuracy = (tp + tn) as f64 / total;
        
        let precision = if tp + fp > 0 {
            tp as f64 / (tp + fp) as f64
        } else {
            0.0
        };
        
        let recall = if tp + fn_count > 0 {
            tp as f64 / (tp + fn_count) as f64
        } else {
            0.0
        };
        
        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Ok(TrainingMetrics {
            accuracy,
            precision,
            recall,
            f1_score,
            train_samples: x_test.nrows(),
            test_samples: y_test.len(),
        })
    }

    /// Sauvegarde le modèle en bincode
    pub fn save_model(
        &self,
        model: &LogisticRegression<f64>,
        path: &Path,
    ) -> Result<(), VolatilityError> {
        // Note: Linfa ne supporte pas directement la sérialisation bincode
        // On sauvegarde les paramètres manuellement
        let params = model.params();
        let serialized = bincode::serialize(&params)
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

        let mut file = File::create(path)
            .map_err(|e| VolatilityError::IoError(e))?;
        
        file.write_all(&serialized)
            .map_err(|e| VolatilityError::IoError(e))?;

        Ok(())
    }

    /// Charge un modèle depuis bincode
    pub fn load_model(
        &self,
        path: &Path,
    ) -> Result<LogisticRegression<f64>, VolatilityError> {
        let mut file = File::open(path)
            .map_err(|e| VolatilityError::IoError(e))?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| VolatilityError::IoError(e))?;

        let params: Array1<f64> = bincode::deserialize(&buffer)
            .map_err(|e| VolatilityError::DatabaseError(e.to_string()))?;

        // Reconstruire le modèle depuis les paramètres
        // Note: Cette approche est simplifiée. En production, utiliser une sérialisation complète
        let model = LogisticRegression::params(params);

        Ok(model)
    }

    /// Retourne la version du modèle
    pub fn model_version(&self) -> &str {
        &self.model_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    fn create_mock_dataframe() -> DataFrame {
        let n = 100;
        let hours: Vec<u8> = (0..n).map(|i| (i % 24) as u8).collect();
        let days: Vec<u8> = (0..n).map(|i| (i % 7) as u8).collect();
        let months: Vec<u8> = (0..n).map(|i| ((i % 12) + 1) as u8).collect();
        let impact_high: Vec<f64> = (0..n).map(|i| if i % 2 == 0 { 1.0 } else { 0.0 }).collect();
        let impact_medium: Vec<f64> = (0..n).map(|i| if i % 3 == 0 { 1.0 } else { 0.0 }).collect();
        let event_economic: Vec<f64> = (0..n).map(|i| if i % 4 == 0 { 1.0 } else { 0.0 }).collect();
        let event_cb: Vec<f64> = (0..n).map(|_| 0.0).collect();
        let has_forecast: Vec<f64> = (0..n).map(|_| 1.0).collect();
        let atr: Vec<f64> = (0..n).map(|i| (i as f64) / 100.0).collect();
        let vol: Vec<f64> = (0..n).map(|i| (i as f64) / 200.0).collect();
        let volume: Vec<f64> = (0..n).map(|i| 1000.0 + (i as f64) * 10.0).collect();
        let body: Vec<f64> = (0..n).map(|i| (i as f64) / 150.0).collect();
        let shadow: Vec<f64> = (0..n).map(|i| (i as f64) / 180.0).collect();
        let target: Vec<f64> = (0..n).map(|i| if i % 5 == 0 { 1.0 } else { 0.0 }).collect();

        DataFrame::new(vec![
            Series::new("hour".into(), hours),
            Series::new("day_of_week".into(), days),
            Series::new("month".into(), months),
            Series::new("impact_high".into(), impact_high),
            Series::new("impact_medium".into(), impact_medium),
            Series::new("event_economic".into(), event_economic),
            Series::new("event_central_bank".into(), event_cb),
            Series::new("has_forecast".into(), has_forecast),
            Series::new("atr_pre_event".into(), atr),
            Series::new("volatility_pre_event".into(), vol),
            Series::new("volume_avg_pre_event".into(), volume),
            Series::new("body_range_avg".into(), body),
            Series::new("shadow_ratio_avg".into(), shadow),
            Series::new("target".into(), target),
        ]).unwrap()
    }

    #[test]
    fn test_trainer_creation() {
        let trainer = MLTrainer::new();
        assert_eq!(trainer.model_version(), "v1.0.0");
    }
