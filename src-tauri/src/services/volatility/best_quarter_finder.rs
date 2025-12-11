// services/volatility/best_quarter_finder.rs - Détection du meilleur quarter (15 min)
// Module séparé pour respecter la limite de taille (metrics.rs < 300L)

use crate::models::Stats15Min;

/// Détecteur du meilleur quarter pour stratégie STRADDLE
pub(super) struct BestQuarterFinder;

impl BestQuarterFinder {
    /// Trouve le meilleur quarter (15 min) pour stratégie STRADDLE (V5 - Breakout Straddle)
    ///
    /// OPTIMISATION STRADDLE (V5) - Breakout, profit sur mouvement directionnel fort:
    /// - Volatilité ÉLEVÉE (50% pondération) = source de profit
    /// - Noise Ratio BAS (20% pondération inverse) = entrées fiables, pas de rejets
    /// - Body Range ÉLEVÉ (15% pondération) = mouvement directionnel clair (Breakout)
    /// - Direction Strength ÉLEVÉE (10% pondération) = tendance forte (Breakout)
    /// - Range (5% pondération) = mouvement exploitable
    ///
    /// Le Straddle V5 cherche le Breakout:
    /// - On veut que le prix PARTE loin du point d'entrée (Grand Corps)
    /// - On veut éviter les mèches (Whipsaw) qui déclenchent les deux ordres puis reviennent
    ///
    /// Retourne (hour, quarter) du meilleur moment de la journée
    pub(super) fn find_best_quarter(stats_15min: &[Stats15Min]) -> Option<(u8, u8)> {
        const VOLATILITY_IDEAL: f64 = 50.0; // 50% = volatilité cible pour Straddle
        const RANGE_IDEAL: f64 = 0.0025; // 25 pips = référence
        const NOISE_IDEAL: f64 = 2.0; // < 2.0 = signal propre

        if stats_15min.is_empty() {
            return None;
        }

        let mut scored_quarters: Vec<((u8, u8), f64)> = stats_15min
            .iter()
            .filter(|q| q.candle_count > 0)
            .map(|q| {
                // ============================================
                // SCORE COMPOSITE STRADDLE (V5 - Breakout)
                // ============================================

                // 1. VOLATILITÉ (50%) - PRIMARY
                // Straddle profite de volatilité élevée
                let volatility_score = (q.volatility_mean / VOLATILITY_IDEAL).min(1.0) * 50.0;

                // 2. NOISE RATIO INVERSE (20%) - LOWER IS BETTER
                // Bruit bas = entrées fiables, moins de fausses mèches
                // Si noise > 3.5 : très mauvais (rejets constants)
                // Si noise < 2.0 : excellent (signal propre)
                let noise_score = if q.noise_ratio_mean < NOISE_IDEAL {
                    // Très bon : +20 pts si bruit < 2.0
                    20.0
                } else if q.noise_ratio_mean < 2.5 {
                    // Bon : +15 pts si bruit 2.0-2.5
                    15.0
                } else if q.noise_ratio_mean < 3.0 {
                    // Acceptable : +10 pts
                    10.0
                } else if q.noise_ratio_mean < 3.5 {
                    // Mauvais : +5 pts
                    5.0
                } else {
                    // Très mauvais : 0 pts, pénalité -10
                    0.0
                };

                // 3. BODY RANGE (15%) - HIGHER IS BETTER FOR BREAKOUT
                // Body Range ÉLEVÉ = mouvement directionnel fort (Breakout réussi)
                // Body Range BAS = indécision (Whipsaw risk)
                // Cible : > 50%
                let body_range_score = (q.body_range_mean / 60.0).min(1.0) * 15.0;

                // 4. DIRECTION STRENGTH (10%) - HIGHER IS BETTER
                // Direction Strength ÉLEVÉE = tendance claire (Breakout)
                // Cible : > 10%
                let direction_score = (q.volume_imbalance_mean / 15.0).min(1.0) * 10.0;

                // 5. RANGE (5%) - SECONDARY
                let range_score = (q.range_mean / RANGE_IDEAL).min(1.0) * 5.0;

                let mut total_score = volatility_score
                    + noise_score
                    + body_range_score
                    + direction_score
                    + range_score;

                // ============================================
                // BONUS / PÉNALITÉS
                // ============================================

                // BONUS: Volatilité excellente (> 50%)
                if q.volatility_mean > 50.0 {
                    total_score += 15.0;
                }

                // BONUS: Breakout clair (Body Range > 50%)
                if q.body_range_mean > 50.0 {
                    total_score += 15.0;
                }

                // BONUS: Signal ultra-propre (noise < 2.0 AND body range > 50%)
                if q.noise_ratio_mean < 2.0 && q.body_range_mean > 50.0 {
                    total_score += 12.0;
                }

                // PÉNALITÉ MAJEURE: Indécision totale (body range < 30%)
                // Risque extrême de Whipsaw (mèches des deux côtés)
                if q.body_range_mean < 30.0 {
                    total_score -= 25.0;
                }

                // PÉNALITÉ: Bruit excessif (noise > 3.5)
                if q.noise_ratio_mean > 3.5 {
                    total_score -= 15.0;
                }

                // PÉNALITÉ: Volatilité trop basse (< 25% = peu de profit)
                if q.volatility_mean < 25.0 {
                    total_score -= 20.0;
                }

                ((q.hour, q.quarter), total_score)
            })
            .collect();

        // Trier par score composite décroissant
        scored_quarters.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Retourner LE meilleur quarter
        scored_quarters.first().map(|(coords, _)| *coords)
    }
}
