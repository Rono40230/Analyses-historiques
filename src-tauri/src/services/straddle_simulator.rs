// services/straddle_simulator.rs
// Simule une stratégie Straddle sur l'historique complet d'un créneau
// AVEC pondération temporelle du whipsaw (Option B)

use crate::models::Candle;
use chrono::Timelike;
use super::straddle_adjustments::AdjustedMetrics;

#[derive(Debug, Clone)]
pub struct StraddleSimulationResult {
    pub total_trades: usize,
    pub wins: usize,
    pub losses: usize,
    pub whipsaws: usize,
    pub win_rate_percentage: f64,
    pub whipsaw_frequency_percentage: f64,
    pub offset_optimal_pips: f64,
    pub percentile_95_wicks: f64,
    pub risk_level: String,
    pub risk_color: String,
    // Valeurs pondérées par le whipsaw (Option B - affichage direct)
    pub win_rate_adjusted: f64,           // Win Rate pondéré par whipsaw
    pub sl_adjusted_pips: f64,            // SL ajusté par whipsaw
    pub trailing_stop_adjusted: f64,      // Trailing Stop réduit
    pub timeout_adjusted_minutes: i32,    // Timeout réduit
}

/// Normalise une valeur en pips selon le symbole
pub fn normalize_to_pips(value: f64, symbol: &str) -> f64 {
    let pip_value = get_pip_value(symbol);
    value / pip_value
}

/// Retourne la valeur d'1 pip pour une paire donnée
pub fn get_pip_value(symbol: &str) -> f64 {
    // Indices
    if symbol.contains("US30") || symbol.contains("DE30") || symbol.contains("NAS100") || symbol.contains("SPX500") {
        return 1.0;
    }
    // Crypto
    if symbol.contains("BTC") {
        return 1.0;
    }
    if symbol.contains("ETH") {
        return 0.1;
    }
    // JPY Pairs
    if symbol.contains("JPY") {
        return 0.01;
    }
    // XAU (Gold)
    if symbol.contains("XAU") {
        return 0.01;
    }
    // Default Forex
    0.0001
}

/// Simule une stratégie Straddle sur un ensemble de bougies avec tracking temporel du whipsaw
/// 
/// Stratégie : Place un ordre Buy Stop et Sell Stop à distance égale du prix d'ouverture
/// Whipsaw pondéré : Chaque whipsaw reçoit un coefficient selon QUAND il se produit
pub fn simulate_straddle(candles: &[Candle], symbol: &str) -> StraddleSimulationResult {
    if candles.is_empty() {
        return StraddleSimulationResult {
            total_trades: 0,
            wins: 0,
            losses: 0,
            whipsaws: 0,
            win_rate_percentage: 0.0,
            whipsaw_frequency_percentage: 0.0,
            offset_optimal_pips: 0.0,
            percentile_95_wicks: 0.0,
            risk_level: "N/A".to_string(),
            risk_color: "#6b7280".to_string(),
            win_rate_adjusted: 0.0,
            sl_adjusted_pips: 0.0,
            trailing_stop_adjusted: 0.0,
            timeout_adjusted_minutes: 0,
        };
    }

    // Calculer le percentile 95 des wicks pour déterminer l'offset optimal
    let mut wicks: Vec<f64> = Vec::new();
    for candle in candles {
        let upper_wick = candle.high - candle.close.max(candle.open);
        let lower_wick = candle.open.min(candle.close) - candle.low;
        if upper_wick > 0.0 {
            wicks.push(upper_wick);
        }
        if lower_wick > 0.0 {
            wicks.push(lower_wick);
        }
    }

    wicks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p95_idx = ((wicks.len() as f64) * 0.95).ceil() as usize;
    let p95_wick = if !wicks.is_empty() && p95_idx < wicks.len() {
        wicks[p95_idx]
    } else {
        0.0
    };

    let offset_optimal = p95_wick * 1.1;
    let offset_optimal_pips = normalize_to_pips(offset_optimal, symbol);

    // Simuler les trades avec pondération temporelle du whipsaw
    let mut total_trades = 0;
    let mut wins = 0;
    let mut losses = 0;
    let mut whipsaws = 0;
    let mut whipsaw_weight_sum = 0.0;

    let tp_distance = offset_optimal * 2.0;
    let sl_distance = offset_optimal;

    for (i, candle) in candles.iter().enumerate() {
        let open = candle.open;
        let high = candle.high;
        let low = candle.low;
        let close = candle.close;
        let entry_time = candle.datetime;

        let buy_stop = open + offset_optimal;
        let sell_stop = open - offset_optimal;

        let mut trade_triggered = false;
        let mut is_win = false;
        let mut is_whipsaw = false;
        let mut whipsaw_duration_minutes = 0;

        // ===== BUY STOP =====
        if high >= buy_stop {
            trade_triggered = true;
            let buy_tp = buy_stop + tp_distance;
            let buy_sl = buy_stop - sl_distance;

            // TOUJOURS chercher la résolution complète dans les 15 minutes suivantes
            // Même si le SL/TP est touché dans la bougie d'entrée, il faut voir ce qui se passe après
            let result = find_trade_resolution(candles, i, entry_time, buy_tp, buy_sl, true);
            is_win = result.0;
            is_whipsaw = result.1;
            whipsaw_duration_minutes = result.2;
        }
        // ===== SELL STOP =====
        else if low <= sell_stop {
            trade_triggered = true;
            let sell_tp = sell_stop - tp_distance;
            let sell_sl = sell_stop + sl_distance;

            // TOUJOURS chercher la résolution complète dans les 15 minutes suivantes
            let result = find_trade_resolution(candles, i, entry_time, sell_tp, sell_sl, false);
            is_win = result.0;
            is_whipsaw = result.1;
            whipsaw_duration_minutes = result.2;
        }

        if trade_triggered {
            total_trades += 1;
            if is_win {
                wins += 1;
            } else {
                losses += 1;
                if is_whipsaw {
                    whipsaws += 1;
                    let coefficient = get_whipsaw_coefficient(whipsaw_duration_minutes);
                    whipsaw_weight_sum += coefficient;
                }
            }
        }
    }

    let win_rate_percentage = if total_trades > 0 {
        (wins as f64 / total_trades as f64) * 100.0
    } else {
        0.0
    };

    let whipsaw_frequency_percentage = if total_trades > 0 {
        (whipsaw_weight_sum / total_trades as f64) * 100.0
    } else {
        0.0
    };

    let (risk_level, risk_color) = calculate_risk_level(whipsaw_frequency_percentage);

    // === CALCUL DES VALEURS PONDÉRÉES PAR LE WHIPSAW (Option B) ===
    let adjusted = AdjustedMetrics::new(
        win_rate_percentage,
        offset_optimal_pips,
        whipsaw_frequency_percentage,
    );

    StraddleSimulationResult {
        total_trades,
        wins,
        losses,
        whipsaws,
        win_rate_percentage,
        whipsaw_frequency_percentage,
        offset_optimal_pips,
        percentile_95_wicks: normalize_to_pips(p95_wick, symbol),
        risk_level,
        risk_color,
        // Valeurs pondérées
        win_rate_adjusted: adjusted.win_rate_adjusted,
        sl_adjusted_pips: adjusted.sl_adjusted_pips,
        trailing_stop_adjusted: adjusted.trailing_stop_adjusted,
        timeout_adjusted_minutes: adjusted.timeout_adjusted_minutes,
    }
}

/// Trouve la résolution d'un trade sur les candles suivantes
/// Retourne: (is_win, is_whipsaw, whipsaw_duration_minutes)
fn find_trade_resolution(
    candles: &[Candle],
    start_idx: usize,
    entry_time: chrono::DateTime<chrono::Utc>,
    tp_level: f64,
    sl_level: f64,
    is_buy: bool,
) -> (bool, bool, i32) {
    use chrono::Duration;
    
    let max_lookforward = std::cmp::min(start_idx + 15, candles.len());

    for check_idx in (start_idx + 1)..max_lookforward {
        let candle = &candles[check_idx];
        
        // Calculer la durée en minutes de manière précise
        let duration = candle.datetime.signed_duration_since(entry_time);
        let duration_minutes = duration.num_minutes() as i32;

        if is_buy {
            if candle.high >= tp_level {
                return (true, false, 0);
            }
            if candle.low <= sl_level {
                return (false, true, duration_minutes);
            }
        } else {
            if candle.low <= tp_level {
                return (true, false, 0);
            }
            if candle.high >= sl_level {
                return (false, true, duration_minutes);
            }
        }
    }

    // Non résolu = loss
    (false, false, 15)
}

/// Retourne le coefficient de pondération selon la durée du whipsaw
/// Option B: Pondération par durée
fn get_whipsaw_coefficient(minutes: i32) -> f64 {
    match minutes {
        0 => 1.0,      // Immédiat = coefficient 1.0 (très grave)
        1..=2 => 0.8,  // 1-2 min = coefficient 0.8
        3..=5 => 0.6,  // 3-5 min = coefficient 0.6
        6..=10 => 0.3, // 6-10 min = coefficient 0.3
        _ => 0.1,      // 11-15 min = coefficient 0.1 (très léger)
    }
}

fn calculate_risk_level(whipsaw_freq_pct: f64) -> (String, String) {
    if whipsaw_freq_pct < 10.0 {
        ("Faible".to_string(), "#22c55e".to_string())
    } else if whipsaw_freq_pct < 20.0 {
        ("Moyen".to_string(), "#eab308".to_string())
    } else if whipsaw_freq_pct < 30.0 {
        ("Élevé".to_string(), "#f97316".to_string())
    } else {
        ("Critique".to_string(), "#ef4444".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pip_value() {
        assert_eq!(get_pip_value("EURUSD"), 0.0001);
        assert_eq!(get_pip_value("BTCUSD"), 1.00);
        assert_eq!(get_pip_value("USDJPY"), 0.01);
    }

    #[test]
    fn test_normalize_to_pips() {
        let value = 0.0020;
        assert_eq!(normalize_to_pips(value, "EURUSD"), 20.0);
    }
}
