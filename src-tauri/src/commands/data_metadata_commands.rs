use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct PairMetadataInfo {
    pub symbol: String,
    pub timeframe: String,
    pub row_count: i32,
    pub last_updated: String,
    pub last_imported_file: String,
    pub quality_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarImportInfo {
    pub id: i32,
    pub name: String,
    pub filename: String,
    pub event_count: i32,
    pub oldest_event_date: Option<String>,
    pub newest_event_date: Option<String>,
    pub imported_at: String,
    pub is_active: bool,
}

/// Récupère les métadonnées des paires depuis pairs.db
#[tauri::command]
pub async fn get_pair_metadata_from_db(
    _pair_state: State<'_, super::pair_data_commands::PairDataState>,
) -> Result<Vec<PairMetadataInfo>, String> {
    use rusqlite::Connection;
    
    tracing::info!("📊 Getting pair metadata from pairs.db...");
    
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("pairs.db");
    
    let conn = Connection::open(&data_dir)
        .map_err(|e| format!("Failed to open pairs.db: {}", e))?;
    
    let mut stmt = conn
        .prepare("SELECT symbol, timeframe, row_count, last_updated, last_imported_file, data_quality_score FROM pair_metadata ORDER BY symbol, timeframe")
        .map_err(|e| format!("Query failed: {}", e))?;
    
    let pairs = stmt
        .query_map([], |row| {
            Ok(PairMetadataInfo {
                symbol: row.get(0)?,
                timeframe: row.get(1)?,
                row_count: row.get(2)?,
                last_updated: row.get(3)?,
                last_imported_file: row.get(4)?,
                quality_score: row.get(5)?,
            })
        })
        .map_err(|e| format!("Query execution failed: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Result collection failed: {}", e))?;
    
    tracing::info!("✅ Found {} pairs in database", pairs.len());
    Ok(pairs)
}

/// Récupère la liste des calendriers importés depuis volatility.db
#[tauri::command]
pub async fn get_calendar_imports_from_db() -> Result<Vec<CalendarImportInfo>, String> {
    use rusqlite::Connection;
    
    tracing::info!("📊 Getting calendar imports from volatility.db...");
    
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");
    
    let conn = Connection::open(&data_dir)
        .map_err(|e| format!("Failed to open volatility.db: {}", e))?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, filename, event_count, oldest_event_date, newest_event_date, imported_at, is_active FROM calendar_imports ORDER BY imported_at DESC")
        .map_err(|e| format!("Query failed: {}", e))?;
    
    let calendars = stmt
        .query_map([], |row| {
            Ok(CalendarImportInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                filename: row.get(2)?,
                event_count: row.get(3)?,
                oldest_event_date: row.get(4)?,
                newest_event_date: row.get(5)?,
                imported_at: row.get(6)?,
                is_active: row.get(7)?,
            })
        })
        .map_err(|e| format!("Query execution failed: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Result collection failed: {}", e))?;
    
    tracing::info!("✅ Found {} calendar imports in database", calendars.len());
    Ok(calendars)
}

/// Récupère l'ID du calendrier actif
#[tauri::command]
#[allow(dead_code)]
pub async fn get_active_calendar_id() -> Result<Option<i32>, String> {
    use rusqlite::Connection;
    
    tracing::info!("🔍 Getting active calendar ID...");
    
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");
    
    let conn = Connection::open(&data_dir)
        .map_err(|e| format!("Failed to open volatility.db: {}", e))?;
    
    let mut stmt = conn
        .prepare("SELECT id FROM calendar_imports WHERE is_active = 1 LIMIT 1")
        .map_err(|e| format!("Query failed: {}", e))?;
    
    let active_id = stmt
        .query_row([], |row| row.get(0))
        .ok();
    
    tracing::info!("🔍 Active calendar ID: {:?}", active_id);
    Ok(active_id)
}

/// Définit l'ID du calendrier actif
#[tauri::command]
#[allow(dead_code)]
pub async fn set_active_calendar_id(calendar_id: i32) -> Result<(), String> {
    use rusqlite::Connection;
    
    tracing::info!("📝 Setting active calendar ID to {}", calendar_id);
    
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");
    
    let mut conn = Connection::open(&data_dir)
        .map_err(|e| format!("Failed to open volatility.db: {}", e))?;
    
    let tx = conn.transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;
    
    // Désactiver tous les autres calendriers
    tx.execute("UPDATE calendar_imports SET is_active = 0", [])
        .map_err(|e| format!("Failed to deactivate calendars: {}", e))?;
    
    // Activer le calendrier sélectionné
    tx.execute("UPDATE calendar_imports SET is_active = 1 WHERE id = ?", [calendar_id])
        .map_err(|e| format!("Failed to activate calendar: {}", e))?;
    
    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;
    
    tracing::info!("✅ Active calendar set to {}", calendar_id);
    Ok(())
}
