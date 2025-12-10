/// Calcul des paramètres Bidi Straddle à partir des données d'impact volatilité
pub struct BidiCalculator;

impl BidiCalculator {
    /// Calcul des 4 paramètres Bidi à partir des données d'impact
    /// Retourne: (meilleur_moment en min, stop_loss en points, trailing_stop coefficient, timeout en min)
    pub fn calculate_from_impact(
        atr_before: &[f64],
        atr_after: &[f64],
        noise_during: f64,
        volatility_increase: f64,
        _event_count: usize,
    ) -> (f64, f64, f64, i32) {
        let best_moment = Self::calculate_best_moment(atr_before);
        let stop_loss = Self::calculate_stop_loss(atr_before, atr_after, noise_during);
        let trailing_stop = Self::calculate_trailing_stop(atr_before, atr_after, noise_during);
        let timeout = Self::calculate_timeout(atr_after, volatility_increase);

        (best_moment, stop_loss, trailing_stop, timeout)
    }

    fn calculate_best_moment(atr_before: &[f64]) -> f64 {
        if atr_before.is_empty() {
            return 0.0;
        }
        let peak_idx = atr_before
            .iter()
            .enumerate()
            .rev()
            .take(5)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(29);
        (29.0 - peak_idx as f64).max(0.0)
    }

    fn calculate_stop_loss(atr_before: &[f64], atr_after: &[f64], noise_during: f64) -> f64 {
        let atr_mean = (atr_before.iter().sum::<f64>() + atr_after.iter().sum::<f64>())
            / (atr_before.len() + atr_after.len()) as f64;

        let sl_ratio = if noise_during > 3.0 {
            3.0
        } else if noise_during > 2.5 {
            2.5
        } else if noise_during > 2.0 {
            2.0
        } else if noise_during > 1.5 {
            1.75
        } else {
            1.5
        };

        (atr_mean * sl_ratio).ceil()
    }

    fn calculate_trailing_stop(atr_before: &[f64], atr_after: &[f64], noise_during: f64) -> f64 {
        let atr_mean = (atr_before.iter().sum::<f64>() + atr_after.iter().sum::<f64>())
            / (atr_before.len() + atr_after.len()) as f64;

        let noise_factor = (noise_during / 3.0).min(1.0);
        let ts_coefficient = 0.75 * (1.0 - noise_factor / 2.0);
        atr_mean * ts_coefficient
    }

    fn calculate_timeout(atr_after: &[f64], volatility_increase: f64) -> i32 {
        let peak_after = atr_after.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let threshold = peak_after * 0.7;
        let mut timeout = 60;

        if peak_after > 0.0 {
            for (i, &atr) in atr_after.iter().enumerate() {
                if atr <= threshold && i > 0 {
                    timeout = (i as i32).min(60);
                    break;
                }
            }
            if timeout == 60 && volatility_increase > 50.0 {
                timeout = 35;
            } else if timeout == 60 && volatility_increase < 10.0 {
                timeout = 50;
            }
        }

        timeout
    }
}
