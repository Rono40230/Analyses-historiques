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
    /// Calcule tous les ajustements basés sur la fréquence whipsaw et la volatilité (ATR)
    /// 
    /// Formules:
    /// 1. Win Rate ajusté = WR brut × (1 - whipsaw_frequency)
    /// 2. SL ajusté = SL brut × (1 + whipsaw_frequency × 0.3)
    /// 3. Trailing Stop ajusté = 1.59 × (1 - whipsaw_frequency / 2)
    /// 4. Timeout ajusté = Basé sur ATR (volatilité)
    ///    - ATR élevé = timeout court (20 min) - volatilité décline vite
    ///    - ATR faible = timeout long (32 min) - volatilité décline lentement
    pub fn new(
        win_rate_percentage: f64,
        offset_optimal_pips: f64,
        whipsaw_frequency_percentage: f64,
        atr_mean: f64,
    ) -> Self {
        let whipsaw_factor = whipsaw_frequency_percentage / 100.0;

        let win_rate_adjusted = win_rate_percentage * (1.0 - whipsaw_factor);

        let sl_adjusted_pips = offset_optimal_pips * (1.0 + whipsaw_factor * 0.3);

        let trailing_stop_brut = 1.59;
        let trailing_stop_adjusted = trailing_stop_brut * (1.0 - whipsaw_factor / 2.0);

        // === TIMEOUT BASÉ SUR LA VOLATILITÉ (ATR) ===
        // Normaliser l'ATR sur une échelle 0.0 - 1.0 (basé sur percentiles typiques)
        // ATR faible typique: 0.0001-0.0003 (Forex)
        // ATR élevée typique: 0.0005-0.0010 (Forex)
        let atr_normalized = (atr_mean / 0.0008).min(1.0); // Normaliser avec 0.0008 comme référence
        
        // Timeout inversement proportionnel à la volatilité:
        // - Volatilité basse (ATR faible) → timeout long (32 min)
        // - Volatilité haute (ATR élevée) → timeout court (18 min)
        let timeout_base = 32.0;
        let timeout_min = 18.0;
        let timeout_adjusted_minutes = 
            (timeout_base - (atr_normalized * (timeout_base - timeout_min))) as i32;

        AdjustedMetrics {
            win_rate_adjusted,
            sl_adjusted_pips,
            trailing_stop_adjusted,
            timeout_adjusted_minutes,
        }
    }
}
