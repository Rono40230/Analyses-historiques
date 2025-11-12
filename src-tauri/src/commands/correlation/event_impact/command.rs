use chrono::{Duration, TimeZone, Utc};
use rusqlite::Connection;
use tauri::State;

use super::calculator::{calculate_pair_impacts, generate_observations};
use super::helpers::{currency_to_country, get_available_pairs};
use super::types::EventImpactResult;
use crate::commands::candle_index_commands::CandleIndexState;
use crate::commands::pair_data::PairDataState;
use crate::services::DatabaseLoader;

#[tauri::command]
pub async fn get_event_impact_by_pair(
    event_type: String,
    event_count: i32,
    calendar_id: Option<i32>,
    _state: State<'_, CandleIndexState>,
    pair_state: State<'_, PairDataState>,
) -> Result<EventImpactResult, String> {
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer");

    let db_path = data_dir.join("volatility.db");

    if !db_path.exists() {
        return Err("Database not found".to_string());
    }

    let conn = Connection::open(&db_path).map_err(|e| format!("Failed to open database: {}", e))?;

    tracing::info!(
        "🔍 get_event_impact_by_pair: searching for description = '{}'",
        event_type
    );

    // Schéma réel: chercher par description avec les colonnes: id, event_time, symbol, impact
    let query = "SELECT id, event_time, symbol, impact 
                 FROM calendar_events 
                 WHERE description = ?1 
                   AND event_time IS NOT NULL
                   AND event_time >= '2024-01-01' 
                 ORDER BY event_time";

    let mut event_stmt = conn
        .prepare(query)
        .map_err(|e| format!("Failed to prepare event query: {}", e))?;

    let events: Vec<(i32, i64, String, String)> = event_stmt
        .query_map([&event_type], |row| {
            // Essayer de parser event_time comme timestamp Unix en millisecondes
            let raw_time: String = row.get::<_, String>(1)?;
            let timestamp_ms: i64 = raw_time.parse().map_err(|_| {
                // Si ce n'est pas un nombre, ignorer l'événement
                rusqlite::Error::InvalidQuery
            })?;
            Ok((row.get(0)?, timestamp_ms, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| format!("Failed to query events: {}", e))?
        .filter_map(|r| r.ok()) // Ignorer les erreurs de parsing
        .collect();

    if calendar_id.is_some() {
        tracing::info!(
            "📊 get_event_impact_by_pair: Requested calendar_id={:?}, but returning all events for backward compatibility",
            calendar_id
        );
    }

    if events.is_empty() {
        return Err(format!("No events found for type: {}", event_type));
    }

    eprintln!("✅ Found {} events for type: {}", events.len(), event_type);
    if let Some((_, first_time_ms, _, _)) = events.first() {
        eprintln!("   First event timestamp: {} ms", first_time_ms);
    }

    let (first_id, first_time_ms, currency, impact_level) = &events[0];
    let country = currency_to_country(currency);

    // Normaliser l'impact pour affichage
    let normalized_impact = match impact_level.as_str() {
        "H" => "HIGH",
        "M" => "MEDIUM",
        "L" => "LOW",
        other => other,
    };

    // Convertir les timestamps en millisecondes en NaiveDateTime
    let first_event_datetime = chrono::DateTime::from_timestamp_millis(*first_time_ms)
        .ok_or_else(|| format!("Invalid timestamp: {} ms", first_time_ms))?
        .naive_utc();

    let window_start = first_event_datetime.format("%Y-%m-%d %H:%M").to_string();
    let window_end = (first_event_datetime + Duration::minutes(120))
        .format("%Y-%m-%d %H:%M")
        .to_string();

    let (_, last_time_ms, _, _) = &events[events.len() - 1];
    let last_event_datetime = chrono::DateTime::from_timestamp_millis(*last_time_ms)
        .ok_or_else(|| format!("Invalid timestamp: {} ms", last_time_ms))?
        .naive_utc();
    let last_datetime_formatted = last_event_datetime.format("%Y-%m-%d %H:%M").to_string();

    let pair_pool = pair_state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pair pool: {}", e))?
        .clone()
        .ok_or("Pair database pool not initialized")?;

    let db_loader = DatabaseLoader::new(pair_pool);
    let mut candle_index =
        crate::services::candle_index::CandleIndex::with_db_loader(db_loader.clone());
    let pairs = get_available_pairs(&db_loader)?;

    for pair in &pairs {
        candle_index.load_pair_candles(pair)?;
    }

    let first_event_utc = Utc.from_utc_datetime(&first_event_datetime);

    let pair_impacts = calculate_pair_impacts(&pairs, &first_event_utc, &candle_index)?;
    let observations = generate_observations(&pair_impacts);

    Ok(EventImpactResult {
        event_id: *first_id,
        event_name: event_type.clone(),
        datetime: first_event_datetime.format("%Y-%m-%d %H:%M").to_string(),
        last_datetime: last_datetime_formatted,
        country,
        currency: currency.clone(),
        impact: normalized_impact.to_string(), // Utiliser impact normalisé
        event_count,
        window_start,
        window_end,
        pair_impacts,
        observations,
    })
}
