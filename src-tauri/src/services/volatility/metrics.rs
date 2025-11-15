// services/volatility/metrics.rs - Calcul des métriques globales et scores
// Conforme .clinerules : < 100L, pas d'unwrap()

use crate::models::{GlobalMetrics, HourlyStats};

/// Calculateur de métriques globales
pub(super) struct MetricsAggregator;

impl MetricsAggregator {
    /// Calcule les métriques globales agrégées
    pub(super) fn calculate_global_metrics(
        hourly_stats: &[HourlyStats],
        total_candles: usize,
    ) -> GlobalMetrics {
        let stats_with_data: Vec<&HourlyStats> =
            hourly_stats.iter().filter(|h| h.candle_count > 0).collect();

        if stats_with_data.is_empty() {
            return GlobalMetrics {
                mean_atr: 0.0,
                mean_volatility: 0.0,
                mean_body_range: 0.0,
                mean_tick_quality: 0.0,
                mean_noise_ratio: 0.0,
                mean_volume_imbalance: 0.0,
                mean_breakout_percentage: 0.0,
                total_candles: 0,
            };
        }

        let count = stats_with_data.len() as f64;

        GlobalMetrics {
            mean_atr: stats_with_data.iter().map(|h| h.atr_mean).sum::<f64>() / count,
            mean_volatility: stats_with_data
                .iter()
                .map(|h| h.volatility_mean)
                .sum::<f64>()
                / count,
            mean_body_range: stats_with_data
                .iter()
                .map(|h| h.body_range_mean)
                .sum::<f64>()
                / count,
            mean_tick_quality: stats_with_data
                .iter()
                .map(|h| h.tick_quality_mean)
                .sum::<f64>()
                / count,
            mean_noise_ratio: stats_with_data
                .iter()
                .map(|h| h.noise_ratio_mean)
                .sum::<f64>()
                / count,
            mean_volume_imbalance: stats_with_data
                .iter()
                .map(|h| h.volume_imbalance_mean)
                .sum::<f64>()
                / count,
            mean_breakout_percentage: stats_with_data
                .iter()
                .map(|h| h.breakout_percentage)
                .sum::<f64>()
                / count,
            total_candles,
        }
    }

    /// Calcule le score de confiance GLOBAL (0-100)
    ///
    /// PHILOSOPHIE :
    /// Ce score mesure "à quel point je peux CONFIER ma stratégie STRADDLE scalping
    /// à cette paire pendant cette période (hourly_stats agrégées)".
    ///
    /// ADAPTATION FOREX M1 :
    /// - Seuils basés sur données 2024 (EURUSD, GBPUSD, cryptos)
    /// - M1 = 1 minute → range petit mais volatilité soutenue = clé
    /// - Scalping = décisions rapides, volatilité CONSTANTE > spike isolé
    ///
    /// FORMULE (max 100 points) :
    ///
    /// 1. ATR (30 pts) - Volatilité soutenue
    ///    >25 pips (0.00025) = 30 pts : volatilité excellente
    ///    15-25 pips = 25 pts
    ///    10-15 pips = 20 pts
    ///    5-10 pips = 10 pts
    ///    → POURQUOI ATR ? Filtre les spikes isolés, mesure volatilité CONSTANTE
    ///
    /// 2. Body Range (25 pts) - Directionnalité des bougies
    ///    >45% = 25 pts : mouvements forts, pas du bruit
    ///    35-45% = 20 pts
    ///    25-35% = 15 pts
    ///    15-25% = 8 pts
    ///    → POURQUOI BodyRange ? Signal/bruit ratio, clé pour scalping
    ///
    /// 3. Volatilité % (25 pts) - BONUS si marché bouge bien
    ///    >30% = 25 pts : crypto-like volatility
    ///    20-30% = 20 pts
    ///    10-20% = 15 pts
    ///    5-10% = 8 pts
    ///    → POURQUOI ce bonus ? Scalping a BESOIN de mouvement
    ///
    /// 4. Noise Ratio (10 pts) - Ratio bruit/signal
    ///    <2.0 = 10 pts : signal propre
    ///    <3.0 = 7 pts
    ///    <4.0 = 4 pts
    ///    → POURQUOI Noise ? Élimine les false signals, confirme signal/bruit
    ///
    /// 5. Breakout % (10 pts) - % de bougies "significatives"
    ///    >15% = 10 pts : beaucoup de vrais mouvements
    ///    >10% = 7 pts
    ///    >5% = 4 pts
    ///    → POURQUOI Breakout ? Scalping veut des CASSURES, pas du sideways
    ///
    /// 6. Bonus Données (5 pts) - Si assez de candles
    ///    >100k candles = 5 pts
    ///    >50k = 3 pts
    ///    → POURQUOI ? Plus de données = plus fiable
    ///
    /// INTERPRÉTATION :
    /// - 80-100 : ✅ EXCELLENT - Scalpe agressivement
    /// - 65-80  : 🟢 BON - Scalpe normalement
    /// - 50-65  : 🟡 PRUDENT - Scalpe avec stop serrés
    /// - 35-50  : 🟠 RISKY - Très prudent, breakouts only
    /// - 0-35   : ❌ MAUVAIS - Ne pas trader
    ///
    /// EXEMPLE : EURUSD 10:00-11:00 UTC
    /// - ATR 0.0003 → 30 pts (excellent volatilité)
    /// - BodyRange 52% → 25 pts (très directif)
    /// - Volatilité 0.25 → 25 pts (bonus mouvement)
    /// - NoiseRatio 1.8 → 10 pts (signal propre)
    /// - BreakoutPct 18% → 10 pts (beaucoup de cassures)
    /// - Bonus → 5 pts (données suffisantes)
    /// = TOTAL 105 → capped à 100 = "EXCELLENT, scalpe agressif"
    pub(super) fn calculate_confidence_score(metrics: &GlobalMetrics) -> f64 {
        let mut score: f64 = 0.0;

        // 1. Score ATR (30 points max) - Seuils adaptés au Forex M1
        // ATR Forex M1 typique : 0.00010 - 0.00030 (10-30 pips)
        if metrics.mean_atr > 0.00025 {
            score += 30.0; // Excellent : >25 pips
        } else if metrics.mean_atr > 0.00015 {
            score += 25.0; // Très bon : 15-25 pips
        } else if metrics.mean_atr > 0.00010 {
            score += 20.0; // Bon : 10-15 pips
        } else if metrics.mean_atr > 0.00005 {
            score += 10.0; // Acceptable : 5-10 pips
        }

        // 2. Score Body Range (25 points max) - Seuils réalistes
        // Body Range Forex : 25-45% est normal, >45% est excellent
        if metrics.mean_body_range > 45.0 {
            score += 25.0; // Excellent : mouvements directionnels forts
        } else if metrics.mean_body_range > 35.0 {
            score += 20.0; // Très bon
        } else if metrics.mean_body_range > 25.0 {
            score += 15.0; // Bon
        } else if metrics.mean_body_range > 15.0 {
            score += 8.0; // Acceptable
        }

        // 3. Score Volatilité (25 points max) - BONUS si volatile
        // Plus c'est volatil, MIEUX c'est pour le scalping !
        if metrics.mean_volatility > 0.30 {
            score += 25.0; // Excellent : cryptos, exotiques
        } else if metrics.mean_volatility > 0.20 {
            score += 20.0; // Très bon : paires majeures volatiles
        } else if metrics.mean_volatility > 0.10 {
            score += 15.0; // Bon : volatilité correcte
        } else if metrics.mean_volatility > 0.05 {
            score += 8.0; // Acceptable
        }

        // 4. Score Noise Ratio (10 points max) - Signal/Bruit
        if metrics.mean_noise_ratio < 2.0 {
            score += 10.0; // Excellent : signal propre
        } else if metrics.mean_noise_ratio < 3.0 {
            score += 7.0; // Bon
        } else if metrics.mean_noise_ratio < 4.0 {
            score += 4.0; // Acceptable
        }

        // 5. Score Breakout % (10 points max) - CRITIQUE pour Straddle
        // % de bougies qui cassent significativement (>P80 ATR)
        if metrics.mean_breakout_percentage > 15.0 {
            score += 10.0; // Excellent : mouvements forts fréquents
        } else if metrics.mean_breakout_percentage > 10.0 {
            score += 7.0; // Très bon
        } else if metrics.mean_breakout_percentage > 5.0 {
            score += 4.0; // Acceptable
        }

        // 6. Bonus données suffisantes (5 points max)
        if metrics.total_candles > 100000 {
            score += 5.0; // Données suffisantes pour fiabilité
        } else if metrics.total_candles > 50000 {
            score += 3.0;
        }

        score.min(100.0)
    }

    /// Trouve les 3 meilleures heures pour trader
    /// Trouve les meilleures heures pour stratégie STRADDLE scalping
    ///
    /// FILTRAGE :
    /// - Retourne TOUTES les heures avec range > 25 pips (seuil straddle)
    /// - Triées par mouvement_potential_score_straddle() décroissant
    /// - Maximum 6 heures (pour couvrir la journée sans surcharger)
    ///
    /// Logique :
    /// 1. Calculer score straddle pour chaque heure
    /// 2. Filtrer : range_mean > 0.0025 (25 pips minimum)
    /// 3. Trier par score décroissant
    /// 4. Retourner top 6 heures qualifiées
    pub(super) fn find_best_hours(hourly_stats: &[HourlyStats]) -> Vec<u8> {
        const STRADDLE_RANGE_THRESHOLD: f64 = 0.0025; // 25 pips minimum
        const MAX_HOURS: usize = 6; // Max heures par jour

        let mut scored_hours: Vec<(u8, f64)> = hourly_stats
            .iter()
            .filter(|h| h.candle_count > 0 && h.range_mean > STRADDLE_RANGE_THRESHOLD)
            .map(|h| (h.hour, h.movement_potential_score_straddle()))
            .collect();

        // Trie par score straddle décroissant
        scored_hours.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Retourne les meilleures heures qualifiées (max 6)
        scored_hours
            .iter()
            .take(MAX_HOURS)
            .map(|(hour, _)| *hour)
            .collect()
    }
}

#[cfg(test)]
mod confidence_tests {
    use super::*;

    #[test]
    fn test_confidence_zero_metrics() {
        let metrics = GlobalMetrics {
            mean_atr: 0.0,
            mean_volatility: 0.0,
            mean_body_range: 0.0,
            mean_noise_ratio: 10.0, // Mauvais
            mean_breakout_percentage: 0.0,
            mean_tick_quality: 0.0,
            mean_volume_imbalance: 0.0,
            total_candles: 1000,
        };

        let score = MetricsAggregator::calculate_confidence_score(&metrics);
        assert!(
            score < 20.0,
            "Mauvaises métriques doivent donner score < 20, obtenu {}",
            score
        );
    }

    #[test]
    fn test_confidence_perfect_metrics() {
        let metrics = GlobalMetrics {
            mean_atr: 0.0003,               // Excellent (>25 pips)
            mean_volatility: 0.35,          // Excellent (>30%)
            mean_body_range: 50.0,          // Excellent (>45%)
            mean_noise_ratio: 1.5,          // Propre (<2.0)
            mean_breakout_percentage: 20.0, // Excellent (>15%)
            mean_tick_quality: 0.001,
            mean_volume_imbalance: 0.05,
            total_candles: 200000,
        };

        let score = MetricsAggregator::calculate_confidence_score(&metrics);
        assert!(
            score >= 80.0,
            "Excellentes métriques doivent donner score >= 80, obtenu {}",
            score
        );
    }

    #[test]
    fn test_confidence_bounds() {
        // Score ne doit JAMAIS dépasser 100
        let test_cases = vec![
            (0.00025, 0.05),
            (0.0001, 0.15),
            (0.0002, 0.30),
            (0.0003, 0.50),
            (0.001, 0.70),
        ];

        for (atr, volatility) in test_cases {
            let metrics = GlobalMetrics {
                mean_atr: atr,
                mean_volatility: volatility,
                mean_body_range: 40.0,
                mean_noise_ratio: 2.0,
                mean_breakout_percentage: 12.0,
                mean_tick_quality: 0.0008,
                mean_volume_imbalance: 0.05,
                total_candles: 100000,
            };

            let score = MetricsAggregator::calculate_confidence_score(&metrics);
            assert!(
                score <= 100.0,
                "Score ne doit pas dépasser 100. ATR={}, Vol={}, Score={}",
                atr,
                volatility,
                score
            );
        }
    }

    #[test]
    fn test_confidence_incremental() {
        // Vérifier que ajouter une bonne métrique augmente le score
        let bad_metrics = GlobalMetrics {
            mean_atr: 0.00003,
            mean_volatility: 0.02,
            mean_body_range: 10.0,
            mean_noise_ratio: 5.0,
            mean_breakout_percentage: 2.0,
            mean_tick_quality: 0.00001,
            mean_volume_imbalance: 0.1,
            total_candles: 10000,
        };

        let good_atr_metrics = GlobalMetrics {
            mean_atr: 0.0003, // Amélioration
            ..bad_metrics.clone()
        };

        let bad_score = MetricsAggregator::calculate_confidence_score(&bad_metrics);
        let good_score = MetricsAggregator::calculate_confidence_score(&good_atr_metrics);

        assert!(
            good_score > bad_score,
            "Améliorer ATR doit augmenter le score"
        );
    }

    #[test]
    fn test_confidence_interpretation() {
        // Vérifier que les seuils correspondent aux interprétations
        let metrics_excellent = GlobalMetrics {
            mean_atr: 0.0003,
            mean_volatility: 0.35,
            mean_body_range: 50.0,
            mean_noise_ratio: 1.5,
            mean_breakout_percentage: 20.0,
            mean_tick_quality: 0.001,
            mean_volume_imbalance: 0.05,
            total_candles: 200000,
        };

        let metrics_good = GlobalMetrics {
            mean_atr: 0.0002,
            mean_volatility: 0.25,
            mean_body_range: 40.0,
            mean_noise_ratio: 2.5,
            mean_breakout_percentage: 12.0,
            mean_tick_quality: 0.0008,
            mean_volume_imbalance: 0.05,
            total_candles: 150000,
        };

        let metrics_prudent = GlobalMetrics {
            mean_atr: 0.00012,
            mean_volatility: 0.12,
            mean_body_range: 30.0,
            mean_noise_ratio: 3.5,
            mean_breakout_percentage: 8.0,
            mean_tick_quality: 0.0005,
            mean_volume_imbalance: 0.1,
            total_candles: 80000,
        };

        let score_excellent = MetricsAggregator::calculate_confidence_score(&metrics_excellent);
        let score_good = MetricsAggregator::calculate_confidence_score(&metrics_good);
        let score_prudent = MetricsAggregator::calculate_confidence_score(&metrics_prudent);

        // Vérifier l'ordre
        assert!(score_excellent > score_good, "Excellent doit être > Good");
        assert!(score_good > score_prudent, "Good doit être > Prudent");

        // Vérifier les catégories
        assert!(score_excellent >= 80.0, "Excellent >= 80");
        assert!(score_good >= 65.0 && score_good < 80.0, "Good entre 65-80");
        assert!(
            score_prudent >= 50.0 && score_prudent < 65.0,
            "Prudent entre 50-65"
        );
    }
}
