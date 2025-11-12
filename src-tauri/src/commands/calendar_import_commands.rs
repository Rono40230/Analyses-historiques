use super::calendar_parser;
use rusqlite::Connection;

/// Importe des fichiers de calendrier économique dans volatility.db
#[tauri::command]
pub async fn import_calendar_files(paths: Vec<String>) -> Result<String, String> {
    tracing::info!("📥 Starting calendar import for {} file(s)", paths.len());

    if paths.is_empty() {
        return Err("Aucun fichier fourni".to_string());
    }

    let path = &paths[0];
    let parsed_events = calendar_parser::parse_calendar_file(path)?;
    let event_count = parsed_events.len();

    if event_count == 0 {
        return Err("Aucun événement valide trouvé dans le fichier".to_string());
    }

    // Calculer les dates min/max
    let mut oldest_date: Option<String> = None;
    let mut newest_date: Option<String> = None;
    for event in &parsed_events {
        let datetime = format!("{} {}", event.date, event.time);
        if oldest_date.as_ref().map(|o| datetime < *o).unwrap_or(true) {
            oldest_date = Some(datetime.clone());
        }
        if newest_date.as_ref().map(|n| datetime > *n).unwrap_or(true) {
            newest_date = Some(datetime);
        }
    }

    tracing::info!("📊 Parsed {} events from CSV", event_count);

    // Ouvrir volatility.db
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("volatility.db");

    let conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open volatility.db: {}", e))?;

    // Extraire le nom du fichier
    let file_path = std::path::Path::new(path);
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file path")?
        .to_string();
    let calendar_name = filename.trim_end_matches(".csv").to_string();

    // Insérer l'enregistrement du calendrier
    let calendar_id: i32 = conn.query_row(
        "INSERT INTO calendar_imports (name, filename, event_count, oldest_event_date, newest_event_date, imported_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         RETURNING id",
        rusqlite::params![&calendar_name, &filename, event_count, &oldest_date, &newest_date, chrono::Utc::now().to_rfc3339()],
        |row| row.get(0),
    )
    .map_err(|e| format!("Failed to insert calendar import record: {}", e))?;

    tracing::info!("📝 Calendar import record created with ID: {}", calendar_id);

    // Insérer les événements
    let mut stmt = conn.prepare(
        "INSERT INTO calendar_events (symbol, event_time, impact, description, created_at, calendar_import_id) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
    .map_err(|e| format!("Failed to prepare insert statement: {}", e))?;

    for event in &parsed_events {
        let datetime = format!("{} {}", event.date, event.time);
        stmt.execute(rusqlite::params![
            &event.currency,
            &datetime,
            &event.impact,
            &event.description,
            chrono::Utc::now().to_rfc3339(),
            calendar_id
        ])
        .map_err(|e| format!("Failed to insert event: {}", e))?;
    }

    tracing::info!(
        "✅ Calendar import complete: {} events imported",
        event_count
    );
    Ok(format!(
        "Calendrier importé avec succès: {} événements",
        event_count
    ))
}
