// services/feature_extractor.rs - Extraction de features pour ML
// Conforme .clinerules : < 300 lignes

use chrono::{Datelike, Timelike, NaiveDateTime};
use polars::prelude::*;
use crate::models::{
    candle::Candle,
    calendar_event::CalendarEvent,
    errors::VolatilityError,
};
use crate::services::MetricsCalculator;

/// Extracteur de features pour le modèle ML
pub struct FeatureExtractor;

/// Structure représentant les features extraites pour un événement
#[derive(Debug, Clone)]
pub struct EventFeatures {
    // Features temporelles
    pub hour: u8,
    pub day_of_week: u8,
    pub month: u8,
    
    // Features calendrier
    pub impact_high: f64,      // 1.0 si HIGH, 0.0 sinon
    pub impact_medium: f64,    // 1.0 si MEDIUM, 0.0 sinon
    pub event_economic: f64,   // 1.0 si Economic, 0.0 sinon
    pub event_central_bank: f64, // 1.0 si CentralBank, 0.0 sinon
    pub has_forecast: f64,     // 1.0 si forecast disponible, 0.0 sinon
    
    // Features volatilité pré-événement (calculées sur 1h avant l'event)
    pub atr_pre_event: f64,
    pub volatility_pre_event: f64,
    pub volume_avg_pre_event: f64,
    pub body_range_avg: f64,
    pub shadow_ratio_avg: f64,
    
    // Target (à prédire)
    pub volatility_spike: f64,  // 1.0 si spike > 50%, 0.0 sinon
}

impl FeatureExtractor {
    /// Crée un nouveau extracteur de features
    pub fn new() -> Self {
        Self
    }

    /// Extrait les features pour un événement donné
    pub fn extract_features(
        &self,
        event: &CalendarEvent,
        candles: &[Candle],
    ) -> Result<EventFeatures, VolatilityError> {
        // Features temporelles
        let hour = event.event_time.hour() as u8;
        let day_of_week = event.event_time.weekday().num_days_from_monday() as u8;
        let month = event.event_time.month() as u8;

        // Features calendrier (one-hot encoding)
        let impact_high = if event.impact_level == "HIGH" { 1.0 } else { 0.0 };
        let impact_medium = if event.impact_level == "MEDIUM" { 1.0 } else { 0.0 };
        let event_economic = if event.event_type == "Economic" { 1.0 } else { 0.0 };
        let event_central_bank = if event.event_type == "CentralBank" { 1.0 } else { 0.0 };
        let has_forecast = if event.forecast_value.is_some() { 1.0 } else { 0.0 };

        // Filtrer les bougies 1h avant l'événement
        let pre_event_candles = self.get_pre_event_candles(event, candles)?;

        // Calculer les métriques de volatilité pré-événement
        let (atr_pre, vol_pre, vol_avg, body_avg, shadow_avg) = 
            self.calculate_pre_event_metrics(&pre_event_candles)?;

        // Calculer le spike de volatilité (target)
        let volatility_spike = self.calculate_volatility_spike(event, candles)?;

        Ok(EventFeatures {
            hour,
            day_of_week,
            month,
            impact_high,
            impact_medium,
            event_economic,
            event_central_bank,
            has_forecast,
            atr_pre_event: atr_pre,
            volatility_pre_event: vol_pre,
            volume_avg_pre_event: vol_avg,
            body_range_avg: body_avg,
            shadow_ratio_avg: shadow_avg,
            volatility_spike,
        })
    }

    /// Récupère les bougies 1h avant l'événement
    fn get_pre_event_candles(
        &self,
        event: &CalendarEvent,
        candles: &[Candle],
    ) -> Result<Vec<Candle>, VolatilityError> {
        let event_time = event.event_time;
        let one_hour_before = event_time - chrono::Duration::hours(1);

        let pre_candles: Vec<Candle> = candles
            .iter()
            .filter(|c| c.datetime.naive_utc() >= one_hour_before && c.datetime.naive_utc() < event_time)
            .cloned()
            .collect();

        if pre_candles.is_empty() {
            return Err(VolatilityError::InsufficientData(
                "Pas de bougies avant l'événement".to_string()
            ));
        }

        Ok(pre_candles)
    }

    /// Calcule les métriques de volatilité pré-événement
    fn calculate_pre_event_metrics(
        &self,
        candles: &[Candle],
    ) -> Result<(f64, f64, f64, f64, f64), VolatilityError> {
        if candles.is_empty() {
            return Ok((0.0, 0.0, 0.0, 0.0, 0.0));
        }

        let calculator = MetricsCalculator::new(candles);
        
        let atr = calculator.calculate_atr(14)
            .map(|v| v.last().copied().unwrap_or(0.0))
            .unwrap_or(0.0);
        
        let volatility = calculator.calculate_volatility(14)
            .map(|v| v.last().copied().unwrap_or(0.0))
            .unwrap_or(0.0);
        
        let volume_avg = candles.iter().map(|c| c.volume).sum::<f64>() / candles.len() as f64;
        
        let body_ranges = calculator.calculate_body_ranges();
        let body_avg = body_ranges.iter().sum::<f64>() / body_ranges.len().max(1) as f64;
        
        let shadow_ratios = calculator.calculate_shadow_ratios();
        let shadow_avg = shadow_ratios.iter().sum::<f64>() / shadow_ratios.len().max(1) as f64;

        Ok((atr, volatility, volume_avg, body_avg, shadow_avg))
    }

    /// Calcule si un spike de volatilité s'est produit après l'événement
    /// Spike = augmentation > 50% de l'ATR dans les 30min suivantes
    fn calculate_volatility_spike(
        &self,
        event: &CalendarEvent,
        candles: &[Candle],
    ) -> Result<f64, VolatilityError> {
        let event_time = event.event_time;
        let thirty_min_after = event_time + chrono::Duration::minutes(30);

        // Bougies avant l'événement
        let pre_candles: Vec<Candle> = candles
            .iter()
            .filter(|c| {
                let t = c.datetime.naive_utc();
                t >= event_time - chrono::Duration::hours(1) && t < event_time
            })
            .cloned()
            .collect();

        // Bougies après l'événement
        let post_candles: Vec<Candle> = candles
            .iter()
            .filter(|c| {
                let t = c.datetime.naive_utc();
                t >= event_time && t < thirty_min_after
            })
            .cloned()
            .collect();

        if pre_candles.len() < 10 || post_candles.len() < 5 {
            return Ok(0.0); // Pas assez de données
        }

        let calc_pre = MetricsCalculator::new(&pre_candles);
        let calc_post = MetricsCalculator::new(&post_candles);
        
        let atr_pre = calc_pre.calculate_atr(14)
            .map(|v| v.last().copied().unwrap_or(0.0))
            .unwrap_or(0.0);
            
        let atr_post = calc_post.calculate_atr(14)
            .map(|v| v.last().copied().unwrap_or(0.0))
            .unwrap_or(0.0);

        if atr_pre == 0.0 {
            return Ok(0.0);
        }

        let increase_pct = ((atr_post - atr_pre) / atr_pre) * 100.0;
        
        // Spike si augmentation > 50%
        Ok(if increase_pct > 50.0 { 1.0 } else { 0.0 })
    }

    /// Convertit un vecteur de EventFeatures en DataFrame Polars
    pub fn to_dataframe(
        &self,
        features: &[EventFeatures],
    ) -> Result<DataFrame, VolatilityError> {
        if features.is_empty() {
            return Err(VolatilityError::InsufficientData(
                "Aucune feature à convertir".to_string()
            ));
        }

        let hours: Vec<u32> = features.iter().map(|f| f.hour as u32).collect();
        let days: Vec<u32> = features.iter().map(|f| f.day_of_week as u32).collect();
        let months: Vec<u32> = features.iter().map(|f| f.month as u32).collect();
        let impact_high: Vec<f64> = features.iter().map(|f| f.impact_high).collect();
        let impact_medium: Vec<f64> = features.iter().map(|f| f.impact_medium).collect();
        let event_economic: Vec<f64> = features.iter().map(|f| f.event_economic).collect();
        let event_cb: Vec<f64> = features.iter().map(|f| f.event_central_bank).collect();
        let has_forecast: Vec<f64> = features.iter().map(|f| f.has_forecast).collect();
        let atr_pre: Vec<f64> = features.iter().map(|f| f.atr_pre_event).collect();
        let vol_pre: Vec<f64> = features.iter().map(|f| f.volatility_pre_event).collect();
        let vol_avg: Vec<f64> = features.iter().map(|f| f.volume_avg_pre_event).collect();
        let body_avg: Vec<f64> = features.iter().map(|f| f.body_range_avg).collect();
        let shadow_avg: Vec<f64> = features.iter().map(|f| f.shadow_ratio_avg).collect();
        let target: Vec<f64> = features.iter().map(|f| f.volatility_spike).collect();

        let df = DataFrame::new(vec![
            Series::new("hour".into(), hours),
            Series::new("day_of_week".into(), days),
            Series::new("month".into(), months),
            Series::new("impact_high".into(), impact_high),
            Series::new("impact_medium".into(), impact_medium),
            Series::new("event_economic".into(), event_economic),
            Series::new("event_central_bank".into(), event_cb),
            Series::new("has_forecast".into(), has_forecast),
            Series::new("atr_pre_event".into(), atr_pre),
            Series::new("volatility_pre_event".into(), vol_pre),
            Series::new("volume_avg_pre_event".into(), vol_avg),
            Series::new("body_range_avg".into(), body_avg),
            Series::new("shadow_ratio_avg".into(), shadow_avg),
            Series::new("target".into(), target),
        ])
        .map_err(|e| VolatilityError::InvalidCsvData(e.to_string()))?;

        Ok(df)
    }

    /// Normalise les features numériques (min-max scaling)
    pub fn normalize_features(
        &self,
        df: &DataFrame,
    ) -> Result<DataFrame, VolatilityError> {
        let numeric_cols = vec![
            "atr_pre_event",
            "volatility_pre_event",
            "volume_avg_pre_event",
            "body_range_avg",
            "shadow_ratio_avg",
        ];

        let mut normalized_df = df.clone();

        for col_name in numeric_cols {
            if let Ok(series) = df.column(col_name) {
                if let Ok(float_series) = series.f64() {
                    let min_val = float_series.min().unwrap_or(0.0);
                    let max_val = float_series.max().unwrap_or(1.0);
