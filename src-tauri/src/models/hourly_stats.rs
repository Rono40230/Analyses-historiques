// models/hourly_stats.rs - Statistiques par heure UTC
use serde::{Deserialize, Serialize};

/// Statistiques de volatilité pour une heure UTC spécifique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyStats {
    pub hour: u8,
    pub candle_count: usize,
    pub atr_mean: f64,
    pub atr_max: f64,
    pub volatility_mean: f64,
    pub range_mean: f64,
    pub body_range_mean: f64,
    pub shadow_ratio_mean: f64,
    pub tick_quality_mean: f64,
    pub volume_imbalance_mean: f64,
    pub noise_ratio_mean: f64,
    pub breakout_percentage: f64,
}

impl HourlyStats {
    /// Calcule un score de qualité global (0-100)
    pub fn quality_score(&self) -> f64 {
        if self.candle_count == 0 { return 0.0; }
        let mut score: f64 = 0.0;
        if self.atr_mean > 0.001 { score += 25.0; } else if self.atr_mean > 0.0005 { score += 15.0; }
        if self.body_range_mean > 50.0 { score += 25.0; } else if self.body_range_mean > 30.0 { score += 15.0; }
        if self.tick_quality_mean > 0.001 { score += 20.0; } else if self.tick_quality_mean > 0.0005 { score += 10.0; }
        if self.noise_ratio_mean < 2.0 { score += 20.0; } else if self.noise_ratio_mean < 3.0 { score += 10.0; }
        if self.breakout_percentage > 20.0 { score += 10.0; }
        score.min(100.0)
    }
    
    #[allow(dead_code)]
    pub fn quality_rating(&self) -> &'static str {
        let score = self.quality_score();
        if score >= 80.0 { "Excellent" } 
        else if score >= 60.0 { "Good" } 
        else if score >= 40.0 { "Fair" } 
        else { "Poor" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_score_calculation() {
        let stats = HourlyStats {
            hour: 13,
            candle_count: 500,
            atr_mean: 0.0015,
            atr_max: 0.003,
            volatility_mean: 0.10,
            range_mean: 0.002,
            body_range_mean: 55.0,
            shadow_ratio_mean: 1.2,
            tick_quality_mean: 0.0012,
            volume_imbalance_mean: 0.15,
            noise_ratio_mean: 1.8,
            breakout_percentage: 25.0,
        };

        let score = stats.quality_score();
        assert!(score >= 0.0 && score <= 100.0);
        assert!(score > 50.0); // Ces bonnes stats devraient donner > 50
    }

    #[test]
    fn test_quality_score_empty() {
        let stats = HourlyStats {
            hour: 0,
            candle_count: 0,
            atr_mean: 0.0,
            atr_max: 0.0,
            volatility_mean: 0.0,
            range_mean: 0.0,
            body_range_mean: 0.0,
            shadow_ratio_mean: 0.0,
            tick_quality_mean: 0.0,
            volume_imbalance_mean: 0.0,
            noise_ratio_mean: 0.0,
            breakout_percentage: 0.0,
        };

        assert_eq!(stats.quality_score(), 0.0);
    }

    #[test]
    fn test_quality_rating() {
        let excellent = HourlyStats {
            hour: 13,
            candle_count: 500,
            atr_mean: 0.002,
            atr_max: 0.004,
            volatility_mean: 0.12,
            range_mean: 0.003,
            body_range_mean: 60.0,
            shadow_ratio_mean: 1.5,
            tick_quality_mean: 0.0015,
            volume_imbalance_mean: 0.2,
            noise_ratio_mean: 1.5,
            breakout_percentage: 30.0,
        };

        assert_eq!(excellent.quality_rating(), "Excellent");
    }
}
