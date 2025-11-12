use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMetadataUI {
    pub id: i32,
    pub name: String,
    pub event_count: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub async fn get_calendar_imports_from_db() -> Result<Vec<CalendarImportInfo>, String> {
    use rusqlite::Connection;

    tracing::info!("📊 Getting calendar imports from volatility.db...");

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");

    let conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open volatility.db: {}", e))?;

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

#[tauri::command]
pub async fn get_calendars_metadata() -> Result<Vec<CalendarMetadataUI>, String> {
    let calendars = get_calendar_imports_from_db().await?;
    Ok(calendars
        .into_iter()
        .map(|c| CalendarMetadataUI {
            id: c.id,
            name: c.name,
            event_count: c.event_count,
            start_date: c.oldest_event_date,
            end_date: c.newest_event_date,
        })
        .collect())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn get_active_calendar_id() -> Result<Option<i32>, String> {
    use rusqlite::Connection;

    tracing::info!("🔍 Getting active calendar ID...");

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");

    let conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open volatility.db: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT id FROM calendar_imports WHERE is_active = 1 LIMIT 1")
        .map_err(|e| format!("Query failed: {}", e))?;

    let active_id = stmt.query_row([], |row| row.get(0)).ok();

    tracing::info!("🔍 Active calendar ID: {:?}", active_id);
    Ok(active_id)
}

#[tauri::command]
#[allow(dead_code)]
pub async fn set_active_calendar_id(calendar_id: i32) -> Result<(), String> {
    use rusqlite::Connection;

    tracing::info!("📝 Setting active calendar ID to {}", calendar_id);

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");

    let mut conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open volatility.db: {}", e))?;

    let tx = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    tx.execute("UPDATE calendar_imports SET is_active = 0", [])
        .map_err(|e| format!("Failed to deactivate calendars: {}", e))?;

    tx.execute(
        "UPDATE calendar_imports SET is_active = 1 WHERE id = ?",
        [calendar_id],
    )
    .map_err(|e| format!("Failed to activate calendar: {}", e))?;

    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    tracing::info!("✅ Active calendar set to {}", calendar_id);
    Ok(())
}

#[tauri::command]
pub async fn get_calendar_id_by_filename(filename: String) -> Result<Option<i32>, String> {
    use rusqlite::Connection;

    tracing::info!("🔍 Getting calendar ID for filename: {}", filename);

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");

    let conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open volatility.db: {}", e))?;

    // D'abord, chercher par exact match
    let mut stmt = conn
        .prepare("SELECT id FROM calendar_imports WHERE filename = ?")
        .map_err(|e| format!("Query failed: {}", e))?;

    if let Ok(calendar_id) = stmt.query_row([&filename], |row| row.get(0)) {
        tracing::info!(
            "🔍 Calendar ID (exact match) for {}: {:?}",
            filename,
            calendar_id
        );
        return Ok(Some(calendar_id));
    }

    // Si pas de match exact, chercher par LIKE (au cas où le format du nom change)
    let search_pattern = format!("%{}%", filename.trim_end_matches(".csv"));
    let mut stmt = conn
        .prepare("SELECT id FROM calendar_imports WHERE filename LIKE ?")
        .map_err(|e| format!("Query failed: {}", e))?;

    if let Ok(calendar_id) = stmt.query_row([&search_pattern], |row| row.get(0)) {
        tracing::info!(
            "🔍 Calendar ID (pattern match) for {}: {:?}",
            filename,
            calendar_id
        );
        return Ok(Some(calendar_id));
    }

    // Si toujours pas trouvé, afficher un warning et retourner None
    tracing::warn!(
        "⚠️ Calendar ID not found for {}. Returning None to use all events.",
        filename
    );
    Ok(None)
}
