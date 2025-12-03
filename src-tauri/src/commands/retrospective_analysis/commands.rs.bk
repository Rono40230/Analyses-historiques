use crate::services::{VolatilityDurationAnalyzer, DatabaseLoader};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{Timelike, Duration};
use rusqlite;
use super::types::{PeakDelayResult, DecayProfileResult, EventTypeList, EventType};

async fn setup_databases(
    state: &tauri::State<'_, crate::commands::calendar_commands::CalendarState>,
) -> Result<(diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>, DatabaseLoader), String> {
    let pool_opt = state.pool.lock().map_err(|_| "DB lock failed")?;
    let pool = pool_opt.as_ref().ok_or("DB not initialized")?;
    let conn = pool.get().map_err(|e| format!("Connection failed: {}", e))?;
    let data_dir = dirs::data_local_dir().ok_or("No data dir")?;
    let pairs_db_url = format!("sqlite://{}", data_dir.join("volatility-analyzer/pairs.db").display());
    let pairs_pool = crate::db::create_pool(&pairs_db_url).map_err(|e| format!("Pool failed: {}", e))?;
    Ok((conn, DatabaseLoader::new(pairs_pool)))
}

async fn load_events_by_type(
    mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    event_type: &str,
) -> Result<Vec<crate::models::CalendarEvent>, String> {
    use crate::db::schema::calendar_events::dsl::*;
    calendar_events
        .filter(description.eq(event_type))
        .order(event_time.asc())
        .load(&mut conn)
        .map_err(|e| format!("Load failed: {}", e))
}

fn calculate_atr(high: f64, low: f64, close: f64) -> f64 {
    (high - low).max((high - close.abs()).max(close - low.abs()))
}

#[tauri::command]
pub async fn analyze_peak_delay(
    pair: String,
    event_type: String,
    state: tauri::State<'_, crate::commands::calendar_commands::CalendarState>,
) -> Result<PeakDelayResult, String> {
    let (conn, loader) = setup_databases(&state).await?;
    let events = load_events_by_type(conn, &event_type).await?;
    if events.is_empty() { return Err(format!("No events: {}", event_type)); }

    let mut peak_delays = Vec::new();
    let mut peak_atrs = Vec::new();
    for event in &events {
        let window_start = event.event_time.and_utc() - Duration::hours(2);
        let window_end = event.event_time.and_utc() + Duration::hours(2);
        let candles = loader.load_candles_by_pair(&pair, "M1", window_start, window_end).unwrap_or_default();
        if !candles.is_empty() {
            let atr_values: Vec<f64> = candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
            if let Ok(pd) = VolatilityDurationAnalyzer::calculate_peak_delay(&atr_values, event.event_time.minute() as u8) {
                peak_delays.push(pd);
            }
            peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        }
    }
    if peak_delays.is_empty() { return Err(format!("Cannot compute peak: {}", pair)); }

    let avg_delay = (peak_delays.iter().sum::<i16>() as f64 / peak_delays.len() as f64) as i16;
    let avg_peak_atr = peak_atrs.iter().sum::<f64>() / peak_atrs.len() as f64;
    Ok(PeakDelayResult {
        peak_delay_minutes: avg_delay,
        peak_atr: avg_peak_atr,
        event_minute: events.first().map(|e| e.event_time.minute() as u8).unwrap_or(0),
        confidence: (peak_delays.len() as f64 / events.len() as f64).min(1.0),
        event_count: events.len(),
        event_type,
        optimal_entry_seconds_before: if avg_peak_atr > 100.0 { 90 } else { 60 },
        event_date_min: events.first().map(|e| e.event_time.to_string()).unwrap_or_default(),
        event_date_max: events.last().map(|e| e.event_time.to_string()).unwrap_or_default(),
    })
}

#[tauri::command]
pub async fn analyze_decay_profile(
    pair: String,
    event_type: String,
    state: tauri::State<'_, crate::commands::calendar_commands::CalendarState>,
) -> Result<DecayProfileResult, String> {
    let (conn, loader) = setup_databases(&state).await?;
    let events = load_events_by_type(conn, &event_type).await?;
    if events.is_empty() { return Err(format!("No events: {}", event_type)); }

    let mut decay_rates = Vec::new();
    let mut peak_atrs = Vec::new();
    for event in &events {
        let window_start = event.event_time.and_utc() - Duration::hours(1);
        let window_end = event.event_time.and_utc() + Duration::hours(3);
        let candles = loader.load_candles_by_pair(&pair, "M1", window_start, window_end).unwrap_or_default();
        if !candles.is_empty() {
            let atr_values: Vec<f64> = candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
            if let Ok((rate, _)) = VolatilityDurationAnalyzer::calculate_decay_profile(&atr_values) {
                decay_rates.push(rate);
            }
            peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        }
    }
    if decay_rates.is_empty() { return Err(format!("Cannot compute decay: {}", pair)); }

    let avg_decay_rate = decay_rates.iter().sum::<f64>() / decay_rates.len() as f64;
    let avg_peak_atr = peak_atrs.iter().sum::<f64>() / peak_atrs.len() as f64;
    let (decay_speed, timeout) = if avg_decay_rate > 3.0 {
        ("Très Rapide".into(), 18)
    } else if avg_decay_rate > 1.5 {
        ("Rapide".into(), 25)
    } else {
        ("Lent".into(), 32)
    };

    Ok(DecayProfileResult {
        peak_atr: avg_peak_atr,
        decay_rate_pips_per_minute: avg_decay_rate,
        decay_speed,
        recommended_timeout_minutes: timeout,
        event_count: events.len(),
        event_type,
    })
}

#[tauri::command]
pub async fn get_event_types(calendar_id: Option<i32>) -> Result<EventTypeList, String> {
    let data_dir = dirs::data_local_dir().ok_or("No data dir")?;
    let db_path = data_dir.join("volatility-analyzer/volatility.db");
    if !db_path.exists() { return Err("DB not found".into()); }

    let conn = rusqlite::Connection::open(&db_path).map_err(|e| format!("Open: {}", e))?;
    let impact_filter = "UPPER(impact) IN ('H', 'HIGH', 'M', 'MEDIUM', 'N')";
    let query = if let Some(cal_id) = calendar_id {
        format!("SELECT description, COUNT(*) FROM calendar_events WHERE {} AND calendar_import_id = {} GROUP BY description ORDER BY COUNT(*) DESC", impact_filter, cal_id)
    } else {
        format!("SELECT description, COUNT(*) FROM calendar_events WHERE {} GROUP BY description ORDER BY COUNT(*) DESC", impact_filter)
    };

    let mut stmt = conn.prepare(&query).map_err(|e| format!("Prep: {}", e))?;
    let types: Vec<EventType> = stmt.query_map([], |row| {
        Ok(EventType { name: row.get(0)?, count: row.get(1)? })
    }).map_err(|e| format!("Query: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Collect: {}", e))?;

    if types.is_empty() && calendar_id.is_some() {
        let fallback = format!("SELECT description, COUNT(*) FROM calendar_events WHERE {} GROUP BY description ORDER BY COUNT(*) DESC", impact_filter);
        let mut stmt = conn.prepare(&fallback).map_err(|e| format!("Fallback: {}", e))?;
        let fallback_types: Vec<EventType> = stmt.query_map([], |row| {
            Ok(EventType { name: row.get(0)?, count: row.get(1)? })
        }).map_err(|e| format!("Fallback query: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Fallback: {}", e))?;
        return if fallback_types.is_empty() { Err("No events".into()) } else { Ok(EventTypeList { types: fallback_types }) };
    }
    
    if types.is_empty() { return Err("No events".into()); }
    Ok(EventTypeList { types })
}
