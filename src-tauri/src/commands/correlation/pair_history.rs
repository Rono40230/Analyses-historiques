use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};

use super::helpers::{parse_sqlite_datetime, calculate_volatility_from_csv, calculate_baseline_volatility_from_csv};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PairEventHistoryItem {
    pub event_id: i32,
    pub datetime: String,
    pub event_name: String,
    pub impact: String,
    pub volatility: f64,
    pub change_percent: f64,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopEvent {
    pub name: String,
    pub datetime: String,
    pub volatility: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairEventHistory {
    pub symbol: String,
    pub period: String,
    pub total_events: i32,
    pub avg_volatility: f64,
    pub max_volatility: f64,
    pub avg_multiplier: f64,
    pub events: Vec<PairEventHistoryItem>,
    pub top_events: Vec<TopEvent>,
}

#[tauri::command]
pub async fn get_pair_event_history(
    pair_symbol: String,
    months_back: Option<i32>,
) -> Result<PairEventHistory, String> {
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
    
    let cutoff_date = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(months as i64 * 30))
        .ok_or("Date calculation error")?
        .format("%Y-%m-%d")
        .to_string();
    
    // Récupérer tous les événements HIGH/MEDIUM dans la période
    let mut event_stmt = conn
        .prepare(
            "SELECT id, description, datetime(event_time), impact
             FROM calendar_events
             WHERE date(event_time) >= ?1 AND impact IN ('HIGH', 'MEDIUM')
             ORDER BY event_time DESC"
        )
        .map_err(|e| format!("Failed to prepare events: {}", e))?;
    
    let events: Vec<(i32, String, String, String)> = event_stmt
        .query_map([&cutoff_date], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| format!("Failed to query events: {}", e))?
        .collect::<SqliteResult<Vec<_>>>()
        .map_err(|e| format!("Failed to collect events: {}", e))?;
    
    let mut event_history = Vec::new();
    let mut total_volatility = 0.0;
    let mut total_multiplier = 0.0;
    let mut max_vol: f64 = 0.0;
    let mut valid_count = 0;
    
    for (event_id, event_name, datetime_str, impact) in &events {
        // Parser la datetime avec fonction robuste
        let event_datetime = match parse_sqlite_datetime(datetime_str) {
            Ok(dt) => dt,
            Err(e) => {
                eprintln!("⚠️ Skipping event {}: {}", event_id, e);
                continue;  // Skip malformed dates
            }
        };
        
        let event_volatility = calculate_volatility_from_csv(&pair_symbol, event_datetime, 30)?;
        let baseline_volatility = calculate_baseline_volatility_from_csv(&pair_symbol, event_datetime, 30)?;
        
        if event_volatility > 0.0 && baseline_volatility > 0.0 {
            let multiplier = event_volatility / baseline_volatility;
            let change_percent = ((event_volatility - baseline_volatility) / baseline_volatility) * 100.0;
            
            let direction = if change_percent > 100.0 {
                "HAUSSIER"
            } else if change_percent > 50.0 {
                "BAISSIER"
            } else {
                "NEUTRE"
            }.to_string();
            
            event_history.push(PairEventHistoryItem {
                event_id: *event_id,
                datetime: datetime_str.clone(),
                event_name: event_name.clone(),
                impact: impact.clone(),
                volatility: event_volatility,
                change_percent,
                direction,
            });
            
            total_volatility += event_volatility;
            total_multiplier += multiplier;
            max_vol = max_vol.max(event_volatility);
            valid_count += 1;
        }
    }
    
    // Calculer le top 5 des événements les plus impactants
    let mut sorted_events = event_history.clone();
    // FIX .clinerules: Remplacer unwrap() par gestion d'erreur explicite
    sorted_events.sort_by(|a, b| {
        b.volatility.partial_cmp(&a.volatility)
            .unwrap_or(std::cmp::Ordering::Equal) // Gère les NaN
    });
    
    let top_events: Vec<TopEvent> = sorted_events
        .iter()
        .take(5)
        .map(|e| TopEvent {
            name: e.event_name.clone(),
            datetime: e.datetime.clone(),
            volatility: e.volatility,
        })
        .collect();
    
    let avg_volatility = if valid_count > 0 { total_volatility / valid_count as f64 } else { 0.0 };
    let avg_multiplier = if valid_count > 0 { total_multiplier / valid_count as f64 } else { 0.0 };
    
    Ok(PairEventHistory {
        symbol: pair_symbol,
        period: format!("{} derniers mois", months),
        total_events: valid_count,
        avg_volatility,
        max_volatility: max_vol,
        avg_multiplier,
        events: event_history,
        top_events,
    })
}
