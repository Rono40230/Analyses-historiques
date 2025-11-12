use crate::services::contextual_atr_analyzer::VolatilityLevel;

/// Configuration pour le calcul des métriques
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub atr_period: usize,
    pub atr_multiplier_sl: f64,
    pub atr_multiplier_tp: f64,
    pub max_trade_duration_minutes: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            atr_period: 14,
            atr_multiplier_sl: 2.0,
            atr_multiplier_tp: 3.0,
            max_trade_duration_minutes: 120,
        }
    }
}

/// Convertit VolatilityLevel en String pour la DB
pub fn volatility_level_to_string(level: &VolatilityLevel) -> String {
    match level {
        VolatilityLevel::Low => "Low".to_string(),
        VolatilityLevel::Medium => "Medium".to_string(),
        VolatilityLevel::High => "High".to_string(),
    }
}
