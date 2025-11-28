// services/straddle_adjustments.rs
// Calculs des valeurs pondérées par le whipsaw

/// Calcule les 4 valeurs ajustées selon la fréquence whipsaw
pub struct AdjustedMetrics {
    pub win_rate_adjusted: f64,        // Win Rate pondéré
    pub sl_adjusted_pips: f64,         // SL ajusté
    pub trailing_stop_adjusted: f64,   // Trailing Stop ajusté
    pub timeout_adjusted_minutes: i32, // Timeout ajusté
}

impl AdjustedMetrics {
    /// Calcule tous les ajustements basés sur la fréquence whipsaw
    /// 
    /// Formules:
    /// 1. Win Rate ajusté = WR brut × (1 - whipsaw_frequency)
    /// 2. SL ajusté = SL brut × (1 + whipsaw_frequency × 0.3)
    /// 3. Trailing Stop ajusté = 1.59 × (1 - whipsaw_frequency / 2)
    /// 4. Timeout ajusté = 32 min × (1 - whipsaw_frequency × 0.5)
    pub fn new(
        win_rate_percentage: f64,
        offset_optimal_pips: f64,
        whipsaw_frequency_percentage: f64,
    ) -> Self {
        let whipsaw_factor = whipsaw_frequency_percentage / 100.0;

        let win_rate_adjusted = win_rate_percentage * (1.0 - whipsaw_factor);

        let sl_adjusted_pips = offset_optimal_pips * (1.0 + whipsaw_factor * 0.3);

        let trailing_stop_brut = 1.59;
        let trailing_stop_adjusted = trailing_stop_brut * (1.0 - whipsaw_factor / 2.0);

        let timeout_brut_minutes = 32;
        let timeout_adjusted_minutes =
            (timeout_brut_minutes as f64 * (1.0 - whipsaw_factor * 0.5)) as i32;

        AdjustedMetrics {
            win_rate_adjusted,
            sl_adjusted_pips,
            trailing_stop_adjusted,
            timeout_adjusted_minutes,
        }
    }
}
