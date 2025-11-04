use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::services::csv_loader::CsvLoader;
use super::helpers::{parse_sqlite_datetime, calculate_volatility_from_csv};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTypeInfo {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeatmapData {
    pub period: String,
    pub pairs: Vec<String>,
    pub event_types: Vec<EventTypeInfo>,
    pub data: HashMap<String, HashMap<String, f64>>,
}

#[tauri::command]
pub async fn get_correlation_heatmap(
    months_back: Option<i32>,
) -> Result<HeatmapData, String> {
    let months = months_back.unwrap_or(6);
    
    // Chemin vers la base de données
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer");
    
    let db_path = data_dir.join("volatility.db");
    
    if !db_path.exists() {
        return Err("Database not found".to_string());
    }
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // 1. Récupérer toutes les paires disponibles
    let pairs = get_available_pairs(&conn)?;
    
    if pairs.is_empty() {
        return Err("No pairs found in database".to_string());
    }
    
    // 2. Récupérer les types d'événements avec leurs occurrences
    let event_types = get_event_types(&conn, months)?;
    
    if event_types.is_empty() {
        return Ok(HeatmapData {
            period: format!("Derniers {} mois", months),
            pairs,
            event_types: vec![],
            data: HashMap::new(),
        });
    }
    
    // 3. Calculer la volatilité moyenne pour chaque combinaison événement × paire
    let mut data: HashMap<String, HashMap<String, f64>> = HashMap::new();
    
    for event_type in &event_types {
        let mut pair_volatilities: HashMap<String, f64> = HashMap::new();
        
        for pair in &pairs {
            let avg_vol = calculate_avg_volatility_for_event_pair(
                &conn,
                &event_type.name,
                pair,
                months,
            )?;
            
            pair_volatilities.insert(pair.clone(), avg_vol);
        }
        
        data.insert(event_type.name.clone(), pair_volatilities);
    }
    
    Ok(HeatmapData {
        period: format!("Derniers {} mois", months),
        pairs,
        event_types,
        data,
    })
}

fn get_available_pairs(_conn: &Connection) -> Result<Vec<String>, String> {
    // Lire les paires disponibles depuis les fichiers CSV au lieu de la DB
    let loader = CsvLoader::new();
    let symbols = loader.list_available_symbols()
        .map_err(|e| format!("Failed to list symbols: {}", e))?;
    
    Ok(symbols)
}

fn get_event_types(conn: &Connection, months_back: i32) -> Result<Vec<EventTypeInfo>, String> {
    // Calculer la date de début (X mois en arrière)
    let cutoff_date = chrono::Utc::now()
        .checked_sub_months(chrono::Months::new(months_back as u32))
        .ok_or("Failed to calculate cutoff date")?
        .format("%Y-%m-%d")
        .to_string();
    
    let mut stmt = conn
        .prepare(
            "SELECT description, COUNT(*) as count 
             FROM calendar_events 
             WHERE date(event_time) >= ?1 
             GROUP BY description 
             HAVING count >= 2
             ORDER BY count DESC, description
             LIMIT 20"
        )
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let event_types = stmt
        .query_map([&cutoff_date], |row| {
            Ok(EventTypeInfo {
                name: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(|e| format!("Failed to query event types: {}", e))?
        .collect::<SqliteResult<Vec<EventTypeInfo>>>()
        .map_err(|e| format!("Failed to collect event types: {}", e))?;
    
    Ok(event_types)
}

fn calculate_avg_volatility_for_event_pair(
    conn: &Connection,
    event_name: &str,
    pair: &str,
    months_back: i32,
) -> Result<f64, String> {
    let cutoff_date = chrono::Utc::now()
        .checked_sub_months(chrono::Months::new(months_back as u32))
        .ok_or("Failed to calculate cutoff date")?
        .format("%Y-%m-%d")
        .to_string();
    
    // Récupérer tous les événements de ce type dans la période
    let mut event_stmt = conn
        .prepare(
            "SELECT datetime(event_time) 
             FROM calendar_events 
             WHERE description = ?1 AND date(event_time) >= ?2
             ORDER BY event_time"
        )
        .map_err(|e| format!("Failed to prepare event statement: {}", e))?;
    
    let events: Vec<String> = event_stmt
        .query_map([event_name, &cutoff_date], |row| {
            row.get::<_, String>(0)
        })
        .map_err(|e| format!("Failed to query events: {}", e))?
        .collect::<SqliteResult<Vec<String>>>()
        .map_err(|e| format!("Failed to collect events: {}", e))?;
    
    if events.is_empty() {
        return Ok(0.0);
    }
    
    let mut total_volatility = 0.0;
    let mut valid_count = 0;
    
    for datetime_str in &events {
        // Parser la datetime avec fonction robuste
        let event_datetime = parse_sqlite_datetime(datetime_str)?;
        
        // Calculer la volatilité autour de cet événement (±30 min)
        let event_volatility = calculate_volatility_from_csv(pair, event_datetime, 30)?;
        
        if event_volatility > 0.0 {
            total_volatility += event_volatility;
            valid_count += 1;
        }
    }
    
    if valid_count == 0 {
        Ok(0.0)
    } else {
        Ok(total_volatility / valid_count as f64)
    }
}
