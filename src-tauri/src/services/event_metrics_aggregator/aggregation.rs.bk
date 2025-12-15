use super::config::MetricsConfig;
use crate::models::{Candle, EventMetrics, Result, VolatilityError};
use chrono::{DateTime, Utc};
use tracing::warn;

/// Calcule les métriques agrégées pour plusieurs occurrences du même événement
pub fn calculate_aggregated_metrics(
    events: Vec<(Vec<Candle>, DateTime<Utc>)>,
    event_name: String,
    config: MetricsConfig,
) -> Result<EventMetrics> {
    if events.is_empty() {
        return Err(VolatilityError::InsufficientData(
            "No events provided for aggregation".to_string(),
        ));
    }

    tracing::info!(
        "Calculating aggregated metrics for {} occurrences of '{}'",
        events.len(),
        event_name
    );

    let mut all_metrics = Vec::new();

    for (candles, event_time) in &events {
        let aggregator =
            super::EventMetricsAggregator::new(candles, *event_time, event_name.clone());

        match aggregator.calculate_all_metrics(config.clone()) {
            Ok(metrics) => all_metrics.push(metrics),
            Err(e) => {
                warn!(
                    "Failed to calculate metrics for event at {}: {}",
                    event_time, e
                );
                continue;
            }
        }
    }

    if all_metrics.is_empty() {
        return Err(VolatilityError::InsufficientData(
            "Could not calculate metrics for any event".to_string(),
        ));
    }

    let n = all_metrics.len() as f64;

    let aggregated = EventMetrics {
        id: None,
        event_name: event_name.clone(),
        event_time: events[0].1,
        symbol: all_metrics[0].symbol.clone(),
        peak_duration_minutes: (all_metrics
            .iter()
            .map(|m| m.peak_duration_minutes as f64)
            .sum::<f64>()
            / n) as i32,
        return_to_normal_minutes: (all_metrics
            .iter()
            .map(|m| m.return_to_normal_minutes as f64)
            .sum::<f64>()
            / n) as i32,
        peak_time_minutes: (all_metrics
            .iter()
            .map(|m| m.peak_time_minutes as f64)
            .sum::<f64>()
            / n) as i64,
        baseline_atr: all_metrics.iter().map(|m| m.baseline_atr).sum::<f64>() / n,
        win_rate: all_metrics.iter().map(|m| m.win_rate).sum::<f64>() / n,
        loss_rate: all_metrics.iter().map(|m| m.loss_rate).sum::<f64>() / n,
        whipsaw_rate: all_metrics.iter().map(|m| m.whipsaw_rate).sum::<f64>() / n,
        risk_reward_ratio: all_metrics.iter().map(|m| m.risk_reward_ratio).sum::<f64>() / n,
        best_entry_minutes_before: (all_metrics
            .iter()
            .map(|m| m.best_entry_minutes_before as f64)
            .sum::<f64>()
            / n) as i32,
        best_entry_win_rate: all_metrics
            .iter()
            .map(|m| m.best_entry_win_rate)
            .sum::<f64>()
            / n,
        worst_entry_minutes_before: (all_metrics
            .iter()
            .map(|m| m.worst_entry_minutes_before as f64)
            .sum::<f64>()
            / n) as i32,
        worst_entry_win_rate: all_metrics
            .iter()
            .map(|m| m.worst_entry_win_rate)
            .sum::<f64>()
            / n,
        atr_before_event: all_metrics.iter().map(|m| m.atr_before_event).sum::<f64>() / n,
        atr_after_event: all_metrics.iter().map(|m| m.atr_after_event).sum::<f64>() / n,
        atr_ratio: all_metrics.iter().map(|m| m.atr_ratio).sum::<f64>() / n,
        max_atr_spike: all_metrics.iter().map(|m| m.max_atr_spike).sum::<f64>() / n,
        recommended_sl_multiplier: all_metrics
            .iter()
            .map(|m| m.recommended_sl_multiplier)
            .sum::<f64>()
            / n,
        recommended_tp_multiplier: all_metrics
            .iter()
            .map(|m| m.recommended_tp_multiplier)
            .sum::<f64>()
            / n,
        baseline_volatility: all_metrics[0].baseline_volatility.clone(),
        sample_size: all_metrics.len() as i32,
        created_at: Utc::now(),
    };

    tracing::info!(
        "Aggregated metrics calculated from {} samples",
        all_metrics.len()
    );
    Ok(aggregated)
}
