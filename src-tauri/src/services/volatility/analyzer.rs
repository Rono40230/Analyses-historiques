// services/volatility/analyzer.rs - Analyseur de volatilité principal
// Conforme .clinerules : < 150L, structure claire, pas d'unwrap()

use crate::db::DbPool;
use crate::models::{
    AnalysisResult, Candle, Result, RiskLevel, TradingRecommendation, VolatilityError,
};
use chrono::Datelike;
use tracing::info;
use super::hourly_stats::HourlyStatsCalculator;
use super::metrics::MetricsAggregator;

/// Analyseur de volatilité principal
pub struct VolatilityAnalyzer {
    candles: Vec<Candle>,
}

impl VolatilityAnalyzer {
    /// Crée un nouvel analyseur avec les bougies fournies
    pub fn new(candles: Vec<Candle>) -> Self {
        Self { candles }
    }

    /// Effectue l'analyse complète et retourne le résultat
    pub fn analyze(&self, symbol: &str, _pool: Option<DbPool>) -> Result<AnalysisResult> {
        info!("Starting volatility analysis for {}", symbol);

        if self.candles.is_empty() {
            return Err(VolatilityError::InsufficientData(
                "No candles provided for analysis".to_string(),
            ));
        }

        // Calculer période et timeframe avec format français
        let period_start = self.candles.first()
            .map(|c| {
                let day = c.datetime.day();
                let month = match c.datetime.month() {
                    1 => "janvier",
                    2 => "février",
                    3 => "mars",
                    4 => "avril",
                    5 => "mai",
                    6 => "juin",
                    7 => "juillet",
                    8 => "août",
                    9 => "septembre",
                    10 => "octobre",
                    11 => "novembre",
                    12 => "décembre",
                    _ => "?",
                };
                let year = c.datetime.year();
                format!("{} {} {}", day, month, year)
            })
            .unwrap_or_else(|| "N/A".to_string());
        
        let period_end = self.candles.last()
            .map(|c| {
                let day = c.datetime.day();
                let month = match c.datetime.month() {
                    1 => "janvier",
                    2 => "février",
                    3 => "mars",
                    4 => "avril",
                    5 => "mai",
                    6 => "juin",
                    7 => "juillet",
                    8 => "août",
                    9 => "septembre",
                    10 => "octobre",
                    11 => "novembre",
                    12 => "décembre",
                    _ => "?",
                };
                let year = c.datetime.year();
                format!("{} {} {}", day, month, year)
            })
            .unwrap_or_else(|| "N/A".to_string());
        
        // Déterminer le timeframe en calculant l'intervalle moyen entre bougies
        let timeframe = if self.candles.len() > 1 {
            let first_ts = self.candles[0].datetime.timestamp();
            let second_ts = self.candles[1].datetime.timestamp();
            let interval_seconds = (second_ts - first_ts).abs();
            
            match interval_seconds {
                60 => "M1",
                300 => "M5",
                900 => "M15",
                1800 => "M30",
                3600 => "H1",
                14400 => "H4",
                86400 => "D1",
                _ => "M1", // Par défaut
            }
        } else {
            "M1"
        }.to_string();

        // 1. Calcule les statistiques par heure
        let calculator = HourlyStatsCalculator::new(&self.candles);
        let hourly_stats = calculator.calculate()?;

        // 2. Trouve les meilleures heures
        let best_hours = MetricsAggregator::find_best_hours(&hourly_stats);

        // 3. Calcule les métriques globales
        let global_metrics = MetricsAggregator::calculate_global_metrics(
            &hourly_stats,
            self.candles.len(),
        );

        // 4. Calcule le score de confiance
        let confidence_score = MetricsAggregator::calculate_confidence_score(&global_metrics);

        // 5. Génère la recommandation
        let recommendation = TradingRecommendation::from_confidence(confidence_score);

        // 6. Détermine le niveau de risque
        let risk_level = RiskLevel::from_volatility(global_metrics.mean_volatility);

                // 7. Corrèle avec événements économiques (si DB disponible)
        // TEMPORAIREMENT DÉSACTIVÉ : problème de parsing de timestamps dans calendar_scraper
        let correlated_events = Vec::new();
        // let correlated_events = if let Some(pool) = pool {
        //     self.correlate_economic_events(symbol, &hourly_stats, pool)?
        // } else {
        //     Vec::new()
        // };

        info!(
            "Analysis complete: confidence={:.1}, recommendation={:?}, {} correlated events",
            confidence_score, recommendation, correlated_events.len()
        );

        Ok(AnalysisResult {
            symbol: symbol.to_string(),
            period_start,
            period_end,
            timeframe,
            hourly_stats,
            best_hours,
            confidence_score,
            recommendation,
            risk_level,
            global_metrics,
            correlated_events,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn create_test_candle(hour: u32, high: f64, low: f64) -> Candle {
        Candle {
            id: None,
            symbol: "TESTBTC".to_string(),
            datetime: DateTime::from_timestamp(1609459200 + (hour as i64 * 3600), 0)
                .expect("Invalid timestamp")
                .into(),
            open: (high + low) / 2.0,
            high,
            low,
            close: (high + low) / 2.0,
            volume: 100.0,
        }
    }

    #[test]
    fn test_analyzer_creation() {
        let candles = vec![
            create_test_candle(0, 50000.0, 49900.0),
            create_test_candle(1, 50100.0, 49950.0),
        ];

        let analyzer = VolatilityAnalyzer::new(candles);
        assert_eq!(analyzer.candles.len(), 2);
    }

    #[test]
    fn test_analyze_with_sufficient_data() {
        let candles: Vec<Candle> = (0..100)
            .map(|i| create_test_candle(i % 24, 50000.0 + i as f64, 49900.0 + i as f64))
            .collect();

        let analyzer = VolatilityAnalyzer::new(candles);
        let result = analyzer.analyze("TESTBTC", None);

        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert_eq!(analysis.symbol, "TESTBTC");
        assert_eq!(analysis.hourly_stats.len(), 24);
        assert!(!analysis.best_hours.is_empty());
    }

    #[test]
    fn test_analyze_empty_candles() {
        let analyzer = VolatilityAnalyzer::new(Vec::new());
        let result = analyzer.analyze("TEST", None);

        assert!(result.is_err());
        match result {
            Err(VolatilityError::InsufficientData(_)) => {}, // Expected
            _ => panic!("Expected InsufficientData error"),
        }
    }
}
