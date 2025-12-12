use crate::models::StraddleParameters;

pub struct StraddleParameterService;

impl StraddleParameterService {
    /// Calcule les paramètres Straddle unifiés (utilisé par Volatilité Brute et Corrélation)
    ///
    /// Logique harmonisée (Bidi V2):
    /// - Offset : Adaptatif selon le bruit (1.2x à 1.5x ATR)
    /// - SL : Adaptatif selon le bruit (1.5x à 3.0x ATR)
    /// - TP : Non défini explicitement (Trailing Stop utilisé)
    /// - Timeout : Basé sur la volatilité (fixe à 3 min pour l'instant ou calculé)
    pub fn calculate_parameters(
        atr: f64,
        noise_ratio: f64,
        point_value: f64,
    ) -> StraddleParameters {
        // 1. Offset Adaptatif
        // Si bruit > 2.0, on s'écarte plus (1.5x) pour éviter les mèches
        // Sinon on reste proche (1.2x)
        let offset_multiplier = if noise_ratio > 2.0 { 1.5 } else { 1.2 };
        let raw_offset = atr * offset_multiplier;
        let offset_pips = (raw_offset / point_value).ceil();

        // 2. Stop Loss Adaptatif (Sécurité)
        // Plus il y a de bruit, plus le SL doit être large
        let sl_ratio = if noise_ratio > 3.0 {
            3.0
        } else if noise_ratio > 2.5 {
            2.5
        } else if noise_ratio > 2.0 {
            2.0
        } else if noise_ratio > 1.5 {
            1.75
        } else {
            1.5
        };
        let raw_sl = atr * sl_ratio;
        let stop_loss_pips = (raw_sl / point_value).ceil();

        // 3. Trailing Stop (Suivi)
        // Environ 30-40% du SL, ou adaptatif
        let ts_ratio = if noise_ratio > 3.0 {
            1.2
        } else if noise_ratio > 2.0 {
            1.0
        } else if noise_ratio > 1.5 {
            0.8
        } else {
            0.6
        };
        let raw_ts = atr * ts_ratio;
        let trailing_stop_pips = (raw_ts / point_value).ceil();

        // 4. SL Recovery (Simultané)
        // Doit couvrir l'offset inverse + marge.
        // Max(SL standard, 3x Offset)
        let sl_recovery_pips = stop_loss_pips.max(offset_pips * 3.0).ceil();

        // 5. Timeout
        // Par défaut 3 minutes pour le scalping/news
        let timeout_minutes = 3;

        // 6. Risk/Reward (Théorique, basé sur volatilité attendue vs SL)
        // Ici on met juste un indicateur
        let risk_reward_ratio = if stop_loss_pips > 0.0 {
            // On vise au moins 1x la volatilité (ATR)
            let target = atr / point_value;
            target / stop_loss_pips
        } else {
            0.0
        };

        StraddleParameters {
            offset_pips,
            stop_loss_pips,
            trailing_stop_pips,
            timeout_minutes,
            sl_recovery_pips,
            risk_reward_ratio,
        }
    }
}
