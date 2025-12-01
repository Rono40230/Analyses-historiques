// services/volatility_duration_analyzer.rs - Analyse décroissance volatilité (<300L)
use crate::models::{Candle, Stats15Min, VolatilityDuration, VolatilityError};

pub struct VolatilityDurationAnalyzer;

impl VolatilityDurationAnalyzer {
    pub fn analyze(slice: &Stats15Min) -> Result<VolatilityDuration, VolatilityError> {
        if slice.candle_count == 0 {
            return Err(VolatilityError::InsufficientData("Créneau vide".to_string()));
        }
        let peak_duration = Self::estimate_peak_duration_heuristic(slice)?;
        let half_life = Self::estimate_half_life_heuristic(slice, peak_duration)?;
        Ok(VolatilityDuration::new(
            slice.hour, slice.quarter, peak_duration, half_life,
            slice.candle_count as u16,
        ))
    }

    fn estimate_peak_duration_heuristic(slice: &Stats15Min) -> Result<u16, VolatilityError> {
        let atr = slice.atr_mean;
        let body_range = slice.body_range_mean;
        let base_duration = if atr > 0.002 && body_range > 50.0 {
            100
        } else if atr > 0.0015 && body_range > 40.0 {
            140
        } else if atr > 0.001 && body_range > 30.0 {
            180
        } else {
            240
        };
        let duration = if !slice.events.is_empty() {
            let max_impact = slice.events.iter().map(|e| e.impact.as_str())
                .max_by_key(|impact| match *impact {
                    "HIGH" => 3, "MEDIUM" => 2, _ => 1,
                }).unwrap_or("MEDIUM");
            match max_impact {
                "HIGH" => (base_duration as f64 * 1.5) as u16,
                "MEDIUM" => base_duration,
                "LOW" => (base_duration as f64 * 0.7) as u16,
                _ => base_duration,
            }
        } else { base_duration };
        Ok(duration.clamp(60, 300))
    }

    fn estimate_half_life_heuristic(
        slice: &Stats15Min,
        peak_duration: u16,
    ) -> Result<u16, VolatilityError> {
        let noise = slice.noise_ratio_mean;
        let ratio = if noise < 1.5 { 0.65 } else if noise < 2.5 { 0.50 } else { 0.35 };
        let half_life = ((peak_duration as f64) * ratio) as u16;
        Ok(half_life.max(30).min((peak_duration as f64 * 0.9) as u16))
    }

    pub fn analyze_from_candles(
        hour: u8, quarter: u8, candles: &[&Candle],
    ) -> Result<VolatilityDuration, VolatilityError> {
        if candles.is_empty() {
            return Err(VolatilityError::InsufficientData("Pas de bougies".to_string()));
        }
        if candles.len() < 5 {
            return Err(VolatilityError::InsufficientData("Min 5 bougies requises".to_string()));
        }
        let atr_values = Self::calculate_atr_values(candles)?;
        let peak_atr = atr_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        if peak_atr <= 0.0 {
            return Err(VolatilityError::MetricCalculationError("ATR peak invalide".to_string()));
        }
        let peak_index = atr_values.iter().position(|&atr| (atr - peak_atr).abs() < 1e-10)
            .unwrap_or(0);
        let peak_duration = Self::calculate_peak_duration(&atr_values, peak_atr)?;
        let half_life = Self::calculate_half_life(&atr_values, peak_index, peak_atr)?;
        let final_half_life = if half_life >= peak_duration {
            (peak_duration as f64 * 0.9) as u16
        } else { half_life };
        Ok(VolatilityDuration::new(hour, quarter, peak_duration, final_half_life,
            candles.len() as u16))
    }

    fn calculate_atr_values(candles: &[&Candle]) -> Result<Vec<f64>, VolatilityError> {
        if candles.len() < 2 {
            return Err(VolatilityError::InsufficientData("Min 2 bougies pour ATR".to_string()));
        }
        let mut atr_values = Vec::new();
        const PERIOD: f64 = 14.0;
        let mut tr_values = Vec::new();
        for i in 0..candles.len() {
            let curr = candles[i];
            let hl = curr.high - curr.low;
            if i == 0 {
                tr_values.push(hl);
            } else {
                let prev_close = candles[i - 1].close;
                let hc = (curr.high - prev_close).abs();
                let lc = (curr.low - prev_close).abs();
                tr_values.push(hl.max(hc).max(lc));
            }
        }
        let mut atr = tr_values.iter().take(14).sum::<f64>() / PERIOD;
        atr_values.push(atr);
        for i in 14..tr_values.len() {
            atr = (atr * (PERIOD - 1.0) + tr_values[i]) / PERIOD;
            atr_values.push(atr);
        }
        Ok(atr_values)
    }

    fn calculate_peak_duration(atr_values: &[f64], peak_atr: f64) -> Result<u16, VolatilityError> {
        let threshold = peak_atr * 0.8;
        let peak_count = atr_values.iter().filter(|&&atr| atr > threshold).count();
        Ok(peak_count.max(1) as u16)
    }

    fn calculate_half_life(
        atr_values: &[f64], peak_index: usize, peak_atr: f64,
    ) -> Result<u16, VolatilityError> {
        let threshold = peak_atr * 0.5;
        for i in (peak_index + 1)..atr_values.len() {
            if atr_values[i] <= threshold {
                return Ok((i - peak_index) as u16);
            }
        }
        let half_life = (atr_values.len() - peak_index) as u16;
        Ok(half_life.max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_candle(hour: u8, minute: u8, close: f64, high: f64, low: f64) -> Candle {
        Candle {
            id: None,
            symbol: "EURUSD".to_string(),
            datetime: Utc::now(),
            open: close,
            high,
            low,
            close,
            volume: 1000.0,
        }
    }

    #[test]
    fn test_analyze_from_candles_typical() {
        let candles = vec![
            create_test_candle(14, 0, 1.0800, 1.0810, 1.0790),
            create_test_candle(14, 1, 1.0805, 1.0815, 1.0795),
            create_test_candle(14, 2, 1.0800, 1.0820, 1.0790),
            create_test_candle(14, 3, 1.0810, 1.0825, 1.0800),
            create_test_candle(14, 4, 1.0820, 1.0830, 1.0810),
            create_test_candle(14, 5, 1.0815, 1.0825, 1.0805),
            create_test_candle(14, 6, 1.0810, 1.0820, 1.0800),
            create_test_candle(14, 7, 1.0805, 1.0815, 1.0795),
            create_test_candle(14, 8, 1.0800, 1.0810, 1.0790),
            create_test_candle(14, 9, 1.0795, 1.0805, 1.0785),
            create_test_candle(14, 10, 1.0790, 1.0800, 1.0780),
            create_test_candle(14, 11, 1.0795, 1.0805, 1.0785),
            create_test_candle(14, 12, 1.0800, 1.0810, 1.0790),
            create_test_candle(14, 13, 1.0805, 1.0815, 1.0795),
            create_test_candle(14, 14, 1.0810, 1.0820, 1.0800),
            create_test_candle(14, 15, 1.0815, 1.0825, 1.0805),
        ];
        let candle_refs: Vec<&Candle> = candles.iter().collect();
        let result = VolatilityDurationAnalyzer::analyze_from_candles(14, 0, &candle_refs);
        assert!(result.is_ok());
        if let Ok(vd) = result {
            assert!(vd.is_valid());
            assert!(vd.peak_duration_minutes > 0);
        }
    }

    #[test]
    fn test_analyze_insufficient_candles() {
        let candles = vec![create_test_candle(14, 0, 1.0800, 1.0810, 1.0790)];
        let candle_refs: Vec<&Candle> = candles.iter().collect();
        assert!(VolatilityDurationAnalyzer::analyze_from_candles(14, 0, &candle_refs).is_err());
    }

    #[test]
    fn test_analyze_empty_candles() {
        let candle_refs: Vec<&Candle> = vec![];
        assert!(VolatilityDurationAnalyzer::analyze_from_candles(14, 0, &candle_refs).is_err());
    }
}
