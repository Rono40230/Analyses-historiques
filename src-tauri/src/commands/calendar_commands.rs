// commands/calendar_commands.rs - Commandes Tauri pour le calendrier
// Conforme .clinerules : < 300 lignes, aucun unwrap()

use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use crate::models::CalendarEvent;
use crate::db::DbPool;

pub struct CalendarState {
    pub pool: Mutex<Option<DbPool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarCommandError {
    pub message: String,
}

impl From<String> for CalendarCommandError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

#[tauri::command]
pub async fn init_calendar_database(
    state: State<'_, CalendarState>,
) -> Result<String, CalendarCommandError> {
    let db_path = dirs::data_local_dir()
        .ok_or_else(|| "Cannot find data directory".to_string())?
        .join("volatility-analyzer")
        .join("calendar.db");
    
    let db_url = format!("sqlite://{}", db_path.display());
    let db_pool = crate::db::create_pool(&db_url)
        .map_err(|e| format!("Failed to create pool: {}", e))?;
    
    crate::db::ensure_calendar_table(&db_pool)
        .map_err(|e| format!("Failed to create table: {}", e))?;
    
    let mut pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    *pool_guard = Some(db_pool);
    Ok("Database initialized".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarInfo {
    pub total_events: usize,
    pub last_import_date: Option<String>,
    pub oldest_event_date: Option<String>,
    pub newest_event_date: Option<String>,
}

#[tauri::command]
pub async fn get_calendar_info(
    state: State<'_, CalendarState>,
) -> Result<CalendarInfo, CalendarCommandError> {
    let pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    let _pool = pool_guard.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    
    Ok(CalendarInfo {
        total_events: 0,
        last_import_date: None,
        oldest_event_date: None,
        newest_event_date: None,
    })
}

#[tauri::command]
pub async fn get_calendar_events(
    state: State<'_, CalendarState>,
) -> Result<Vec<CalendarEvent>, CalendarCommandError> {
    let pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    let _pool = pool_guard.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    
    Ok(vec![])
}

#[tauri::command]
pub async fn get_upcoming_events(
    state: State<'_, CalendarState>,
) -> Result<Vec<CalendarEvent>, CalendarCommandError> {
    let pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    let _pool = pool_guard.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    
    Ok(vec![])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictedEvent {
    pub id: i32,
    pub currency: String,
    pub event: String,
    pub impact: String,
    pub datetime: chrono::NaiveDateTime,
    pub predicted_volatility_increase: f64,
}

#[tauri::command]
pub async fn predict_calendar_events(
    state: State<'_, CalendarState>,
) -> Result<Vec<PredictedEvent>, CalendarCommandError> {
    let pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    let _pool = pool_guard.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    
    Ok(vec![])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingResult {
    pub mae: f64,
    pub rmse: f64,
    pub r2: f64,
    pub train_samples: usize,
    pub test_samples: usize,
    pub model_version: String,
}

#[tauri::command]
pub async fn train_ml_model(
    state: State<'_, CalendarState>,
) -> Result<TrainingResult, CalendarCommandError> {
    let pool_guard = state.pool.lock()
        .map_err(|e| format!("Failed to lock pool: {}", e))?;
    
    let _pool = pool_guard.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;
    
    Err("ML training not yet implemented".to_string().into())
}
