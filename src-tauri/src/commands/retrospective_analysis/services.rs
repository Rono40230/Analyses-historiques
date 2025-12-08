use super::types::{DecayProfileResult, DecayProfileDetailedResult, PeakDelayResult, VolatilityProfileResult};
use super::helpers::{setup_databases, load_events_by_type, calculate_atr};
use crate::services::VolatilityDurationAnalyzer;
use chrono::Duration;

pub struct RetroAnalysisService;

impl RetroAnalysisService {
    pub async fn compute_peak_delay(
        pair: &str,
        event_type: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::db::CandleLoader,
    ) -> Result<(Vec<i16>, Vec<f64>), String> {
        let mut peak_delays = Vec::new();
        let mut peak_atrs = Vec::new();

        for event in events {
            let window_start = event.event_time.and_utc() - Duration::hours(2);
            let window_end = event.event_time.and_utc() + Duration::hours(2);
            let candles = loader
                .load_candles_by_pair(pair, "M1", window_start, window_end)
                .unwrap_or_default();

            if !candles.is_empty() {
                let atr_values: Vec<f64> =
                    candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
                if let Ok(pd) = VolatilityDurationAnalyzer::calculate_peak_delay(
                    &atr_values,
                    event.event_time.minute() as u8,
                ) {
                    peak_delays.push(pd);
                }
                peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
            }
        }

        if peak_delays.is_empty() {
            return Err(format!("Cannot compute peak: {}", pair));
        }

        Ok((peak_delays, peak_atrs))
    }

    pub async fn compute_decay_profile(
        pair: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::db::CandleLoader,
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        let mut decay_rates = Vec::new();
        let mut peak_atrs = Vec::new();

        for event in events {
            let window_start = event.event_time.and_utc() - Duration::hours(1);
            let window_end = event.event_time.and_utc() + Duration::hours(3);
            let candles = loader
                .load_candles_by_pair(pair, "M1", window_start, window_end)
                .unwrap_or_default();

            if !candles.is_empty() {
                let atr_values: Vec<f64> =
                    candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
                if let Ok((rate, _)) = VolatilityDurationAnalyzer::calculate_decay_profile(&atr_values)
                {
                    decay_rates.push(rate);
                }
                peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
            }
        }

        if decay_rates.is_empty() {
            return Err(format!("Cannot compute decay: {}", pair));
        }

        Ok((decay_rates, peak_atrs))
    }

    pub async fn compute_volatility_profile(
        pair: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::db::CandleLoader,
    ) -> Result<(Vec<f64>, f64, f64, f64, i32), String> {
        let mut all_atr_series: Vec<Vec<f64>> = Vec::new();

        for event in events {
            let window_start = event.event_time.and_utc() - Duration::minutes(30);
            let window_end = event.event_time.and_utc() + Duration::minutes(90);
            let candles = loader
                .load_candles_by_pair(pair, "M1", window_start, window_end)
                .unwrap_or_default();

            if !candles.is_empty() {
                let atr_values: Vec<f64> =
                    candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
                all_atr_series.push(atr_values);
            }
        }

        if all_atr_series.is_empty() {
            return Err(format!("No candle data for: {}", pair));
        }

        // Calculer ATR5 glissant
        let max_len = all_atr_series.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut atr5_timeline = vec![0.0; 121];
        let mut peak_atr5 = 0.0;
        let mut peak_minute = 0i32;

        for minute in 0..121 {
            let mut values = Vec::new();
            for series in &all_atr_series {
                if minute < series.len() {
                    values.push(series[minute]);
                }
            }
            if !values.is_empty() {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                atr5_timeline[minute] = avg;
                if avg > peak_atr5 {
                    peak_atr5 = avg;
                    peak_minute = minute as i32;
                }
            }
        }

        let mean_atr5 = atr5_timeline.iter().sum::<f64>() / atr5_timeline.len() as f64;
        let variance = atr5_timeline
            .iter()
            .map(|v| (v - mean_atr5).powi(2))
            .sum::<f64>()
            / atr5_timeline.len() as f64;
        let std_atr5 = variance.sqrt();

        Ok((atr5_timeline, peak_atr5, mean_atr5, std_atr5, peak_minute))
    }
}
