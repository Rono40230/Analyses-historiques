// commands/economic_commands.rs
use crate::commands::calendar_commands::CalendarState;
use crate::models::CalendarEvent;
use crate::services::{
    CalendarConverter, CorrelationStats, EconomicEventLoader, EventCorrelationService,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarImportInfo {
    pub total_events: usize,
    pub last_import_date: Option<String>,
    pub oldest_event_date: Option<String>,
    pub newest_event_date: Option<String>,
}

#[tauri::command]
pub async fn import_and_convert_calendar(
    source_path: String,
    state: State<'_, CalendarState>,
) -> Result<usize, String> {
    tracing::info!("🚀 Début import automatique: {}", source_path);
    let conversion_result = CalendarConverter::convert_file(&source_path)
        .map_err(|e| format!("Erreur de conversion: {}", e))?;
    tracing::info!(
        "✅ Conversion: {} événements filtrés",
        conversion_result.total_filtered
    );
    let save_path = CalendarConverter::get_standard_save_path(&conversion_result.events)
        .map_err(|e| format!("Erreur chemin sauvegarde: {}", e))?;
    tracing::info!("💾 Sauvegarde dans: {}", save_path);
    CalendarConverter::save_to_csv(&conversion_result.events, &save_path)
        .map_err(|e| format!("Erreur de sauvegarde: {}", e))?;
    tracing::info!("✅ Fichier sauvegardé dans: {}", save_path);
    tracing::info!("📥 Import dans la base de données...");
    let pool_guard = state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    let pool = pool_guard.as_ref().ok_or("Database pool not initialized")?;
    crate::db::ensure_calendar_table(pool).map_err(|e| format!("Erreur création table: {}", e))?;
    let loader = EconomicEventLoader::new(pool.clone());
    let count = loader
        .load_from_csv(&save_path)
        .map_err(|e| format!("Erreur base de données: {}", e))?;
    tracing::info!("✅ {} événements importés", count);
    if let Err(e) = fs::remove_file(&source_path) {
        tracing::warn!("⚠️  Impossible de supprimer {}: {}", source_path, e);
    }
    tracing::info!("🎉 Import automatique terminé: {} événements", count);
    Ok(count)
}

#[tauri::command]
pub async fn load_economic_events_from_csv(
    csv_path: String,
    state: State<'_, CalendarState>,
) -> Result<usize, String> {
    tracing::info!("Command: load_economic_events_from_csv({})", csv_path);
    let pool_guard = state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    let pool = pool_guard.as_ref().ok_or("Database pool not initialized")?;
    let loader = EconomicEventLoader::new(pool.clone());
    let count = loader
        .load_from_csv(&csv_path)
        .map_err(|e| format!("Failed to load CSV: {}", e))?;
    tracing::info!("Successfully imported {} events from CSV", count);
    Ok(count)
}

#[tauri::command]
pub async fn get_calendar_import_info(
    state: State<'_, CalendarState>,
) -> Result<CalendarImportInfo, String> {
    use crate::db::schema::calendar_events::dsl::*;
    let pool_guard = state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    let pool = pool_guard.as_ref().ok_or("Database pool not initialized")?;
    let mut conn = pool
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e))?;
    let total_count: i64 = calendar_events.count().get_result(&mut conn).unwrap_or(0);
    let last_import: Option<NaiveDateTime> = calendar_events
        .select(diesel::dsl::max(created_at))
        .first(&mut conn)
        .ok()
        .flatten();
    let oldest_event: Option<NaiveDateTime> = calendar_events
        .select(diesel::dsl::min(event_time))
        .first(&mut conn)
        .ok()
        .flatten();
    let newest_event: Option<NaiveDateTime> = calendar_events
        .select(diesel::dsl::max(event_time))
        .first(&mut conn)
        .ok()
        .flatten();
    Ok(CalendarImportInfo {
        total_events: total_count as usize,
        last_import_date: last_import.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        oldest_event_date: oldest_event.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        newest_event_date: newest_event.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
    })
}

#[tauri::command]
pub async fn get_events_for_period(
    symbol: String,
    start_time: i64,
    end_time: i64,
    state: State<'_, CalendarState>,
) -> Result<Vec<CalendarEvent>, String> {
    use chrono::TimeZone;
    let pool_guard = state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    let pool = pool_guard.as_ref().ok_or("Database pool not initialized")?;
    let service = EventCorrelationService::new(pool.clone());
    let start_dt = chrono::Utc
        .timestamp_opt(start_time, 0)
        .single()
        .ok_or("Invalid start timestamp")?
        .naive_utc();
    let end_dt = chrono::Utc
        .timestamp_opt(end_time, 0)
        .single()
        .ok_or("Invalid end timestamp")?
        .naive_utc();
    service
        .get_events_for_period(&symbol, start_dt, end_dt)
        .map_err(|e| format!("Failed to get events: {}", e))
}

#[tauri::command]
pub async fn analyze_event_correlation(
    symbol: String,
    days_back: i64,
    state: State<'_, CalendarState>,
) -> Result<CorrelationStats, String> {
    let _ = (symbol, days_back, state);
    Err("Not implemented: Cette fonction nécessite des données de bougies qui ne sont pas disponibles dans ce contexte".to_string())
}
