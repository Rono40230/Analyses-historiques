use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PastEvent {
    pub id: i32,
    pub name: String,
    pub datetime: String,
    pub country: String,
    pub currency: String,
    pub impact: String,
}

#[tauri::command]
pub async fn get_past_events(months_back: Option<i32>) -> Result<Vec<PastEvent>, String> {
    let months = months_back.unwrap_or(6);
    
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer");
    
    let db_path = data_dir.join("volatility.db");
    
    if !db_path.exists() {
        return Err("Database not found".to_string());
    }
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Calculer la date limite (ex: 6 mois en arrière)
    let cutoff_date = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(months as i64 * 30))
        .ok_or("Date calculation error")?
        .format("%Y-%m-%d")
        .to_string();
    
    let mut stmt = conn
        .prepare(
            "SELECT id, description, datetime(event_time), symbol, impact 
             FROM calendar_events 
             WHERE date(event_time) >= ?1 
             ORDER BY event_time DESC"
        )
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let events_iter = stmt
        .query_map([&cutoff_date], |row| {
            Ok(PastEvent {
                id: row.get(0)?,
                name: row.get(1)?,
                datetime: row.get(2)?,
                country: "Unknown".to_string(), // Pas de colonne country
                currency: row.get(3)?,
                impact: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query events: {}", e))?;
    
    let events: Vec<PastEvent> = events_iter
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| format!("Failed to collect events: {}", e))?;
    
    Ok(events)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTypeInfo {
    pub event_type: String,
    pub count: i32,
}

#[tauri::command]
pub async fn get_event_types_command() -> Result<Vec<EventTypeInfo>, String> {
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer");
    
    let db_path = data_dir.join("volatility.db");
    
    if !db_path.exists() {
        return Err("Database not found".to_string());
    }
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    let mut stmt = conn
        .prepare("SELECT impact, COUNT(*) as count FROM calendar_events GROUP BY impact ORDER BY count DESC")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let types_iter = stmt
        .query_map([], |row| {
            Ok(EventTypeInfo {
                event_type: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(|e| format!("Failed to query event types: {}", e))?;
    
    let types: Vec<EventTypeInfo> = types_iter
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| format!("Failed to collect event types: {}", e))?;
    
    Ok(types)
}
