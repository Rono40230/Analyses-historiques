// services/volatility/best_hours_finder.rs - Détection des meilleures heures
// Module séparé pour respecter la limite de taille (metrics.rs < 300L)

use crate::models::HourlyStats;

/// Détecteur des meilleures heures pour stratégie STRADDLE
pub(super) struct BestHoursFinder;

impl BestHoursFinder {
    /// Trouve les meilleures heures pour stratégie STRADDLE (V3)
    ///
    /// OPTIMISATION STRADDLE (V3) :
    /// - Priorité ABSOLUE à la Volatilité (40% pondération)
    /// - Range + ATR secondaires (30% + 20%)
    /// - Direction Strength (10%)
    /// - Pénalité pour Whipsaw (volatilité chaotique)
    /// - Straddle cherche VOLATILITÉ ÉLEVÉE, pas juste "signal propre"
    ///
    /// Logique :
    /// 1. Calculer score composite pour chaque heure
    ///    - Volatilité % (40% pondération) ← NOUVEAU, PRIORITAIRE
    ///    - Range (30%)
    ///    - ATR (20%)
    ///    - Direction Strength (10%)
    /// 2. Bonus si volatilité > 25% (conditions excellentes)
    /// 3. Pénalité si volatilité < 15% (trop calme pour straddle)
    /// 4. Retourner top 6 heures avec score le plus élevé
    pub(super) fn find_best_hours(hourly_stats: &[HourlyStats]) -> Vec<u8> {
        const MAX_HOURS: usize = 6;
        const RANGE_IDEAL: f64 = 0.0025; // 25 pips = référence 100%
        const ATR_IDEAL: f64 = 0.0020;
        const VOLATILITY_IDEAL: f64 = 25.0; // 25% = référence 100% pour straddle (volatility_mean est en pourcentage 0-100)
        const DIRECTION_STRENGTH_IDEAL: f64 = 20.0; // 20% = référence 100% (en format pourcentage)

        let mut scored_hours: Vec<(u8, f64)> = hourly_stats
            .iter()
            .filter(|h| h.candle_count > 0)
            .map(|h| {
                // Score composite STRADDLE : Volatilité (40%) + Range (30%) + ATR (20%) + Direction (10%)
                // NOTE: volatility_mean et volume_imbalance_mean sont en format pourcentage (0-100)
                let volatility_score = (h.volatility_mean / VOLATILITY_IDEAL).min(1.0) * 40.0;
                let range_score = (h.range_mean / RANGE_IDEAL).min(1.0) * 30.0;
                let atr_score = (h.atr_mean / ATR_IDEAL).min(1.0) * 20.0;
                let direction_score = (h.volume_imbalance_mean / DIRECTION_STRENGTH_IDEAL).min(1.0) * 10.0;

                let mut total_score = volatility_score + range_score + atr_score + direction_score;

                // BONUS: Volatilité excellente (> 25% = parfait pour straddle)
                if h.volatility_mean > 25.0 {
                    total_score += 20.0;
                } else if h.volatility_mean > 20.0 {
                    // Volatilité très bonne (20-25%)
                    total_score += 10.0;
                }

                // PÉNALITÉ MAJEURE: Volatilité trop faible (< 15% = trop calme pour straddle)
                // Straddle ne profite PAS d'un marché endormi
                if h.volatility_mean < 15.0 {
                    total_score -= 30.0; // Pénalité sévère
                }

                // PÉNALITÉ: Volatilité chaotique (Breakout % très élevé + BodyRange faible)
                // = fausses cassures = Whipsaw élevé = danger pour straddle
                if h.breakout_percentage > 20.0 && h.body_range_mean < 25.0 {
                    total_score -= 12.0;
                }

                // PÉNALITÉ: Noise ratio très élevé (> 3.5 = trop de rejet)
                if h.noise_ratio_mean > 3.5 {
                    total_score -= 10.0;
                }

                // BONUS: Signal propre avec haute volatilité (idéal straddle)
                if h.noise_ratio_mean < 2.0 && h.volatility_mean > 20.0 {
                    total_score += 15.0;
                }

                (h.hour, total_score)
            })
            .collect();

        // Trier par score composite décroissant
        scored_hours.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Retourner les meilleures heures (max 6)
        scored_hours
            .iter()
            .take(MAX_HOURS)
            .map(|(hour, _)| *hour)
            .collect()
    }
}
