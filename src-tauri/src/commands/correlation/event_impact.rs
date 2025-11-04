use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use chrono::Duration;

use super::helpers::{parse_sqlite_datetime, calculate_volatility_from_csv, calculate_baseline_volatility_from_csv};

fn get_available_pairs(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT DISTINCT symbol FROM calendar_events")
        .map_err(|e| format!("Failed to get pairs: {}", e))?;
    
    let pairs: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("Failed to query pairs: {}", e))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| format!("Failed to collect pairs: {}", e))?;
    
    Ok(pairs)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairImpactDetail {
    pub symbol: String,
    pub event_volatility: f64,
    pub baseline_volatility: f64,
    pub multiplier: f64,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventImpactResult {
    pub event_id: i32,
    pub event_name: String,
    pub datetime: String,
    pub country: String,
    pub currency: String,
    pub impact: String,
    pub window_start: String,
    pub window_end: String,
    pub pair_impacts: Vec<PairImpactDetail>,
    pub observations: Vec<String>,
}

#[tauri::command]
pub async fn get_event_impact_by_pair(event_id: i32) -> Result<EventImpactResult, String> {
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer");
    
    let db_path = data_dir.join("volatility.db");
    
    if !db_path.exists() {
        return Err("Database not found".to_string());
    }
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Récupérer les détails de l'événement
    let mut event_stmt = conn
        .prepare("SELECT description, datetime(event_time), symbol, impact FROM calendar_events WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare event query: {}", e))?;
    
    let (event_name, datetime, currency, impact): (String, String, String, String) = 
        event_stmt
            .query_row([event_id], |row| {
                Ok((
                    row.get(0)?,  // description
                    row.get(1)?,  // event_time formaté
                    row.get(2)?,  // symbol
                    row.get(3)?,  // impact
                ))
            })
            .map_err(|e| format!("Event not found: {}", e))?;
    
    let country = "Unknown".to_string();  // Pas de colonne country
    
    // Parser la datetime de l'événement avec fonction robuste
    let event_datetime = parse_sqlite_datetime(&datetime)?;
    
    // Obtenir toutes les paires disponibles
    let pairs = get_available_pairs(&conn)?;
    
    let mut pair_impacts = Vec::new();
    
    for pair in &pairs {
        // Calculer la volatilité pendant l'événement (±30 min)
        let event_volatility = calculate_volatility_from_csv(pair, event_datetime, 30)?;
        
        // Calculer la volatilité baseline (30 jours)
        let baseline_volatility = calculate_baseline_volatility_from_csv(pair, event_datetime, 30)?;
        
        let multiplier = if baseline_volatility > 0.0 {
            event_volatility / baseline_volatility
        } else {
            0.0
        };
        
        // Déterminer la direction (simplifié pour le moment)
        let direction = if multiplier > 10.0 {
            "HAUSSIER".to_string()
        } else if multiplier > 5.0 {
            "BAISSIER".to_string()
        } else {
            "NEUTRE".to_string()
        };
        
        pair_impacts.push(PairImpactDetail {
            symbol: pair.clone(),
            event_volatility,
            baseline_volatility,
            multiplier,
            direction,
        });
    }
    
    // Trier par multiplicateur décroissant
    // FIX .clinerules: Remplacer unwrap() par gestion d'erreur explicite
    pair_impacts.sort_by(|a, b| {
        b.multiplier.partial_cmp(&a.multiplier)
            .unwrap_or(std::cmp::Ordering::Equal) // Gère les NaN
    });
    
    // Générer des observations basées sur les données
    let mut observations = Vec::new();
    
    if let Some(top_pair) = pair_impacts.first() {
        observations.push(format!(
            "{} a enregistré le plus fort impact avec {:.0} pips, soit {:.1}× sa volatilité normale",
            top_pair.symbol, top_pair.event_volatility, top_pair.multiplier
        ));
    }
    
    let high_impact_count = pair_impacts.iter().filter(|p| p.multiplier > 5.0).count();
    if high_impact_count > 0 {
        observations.push(format!(
            "{} paires ont réagi significativement (multiplicateur >5×)",
            high_impact_count
        ));
    }
    
    // Calculer les bornes de la fenêtre d'analyse
    let window_start = (event_datetime - Duration::minutes(30))
        .format("%Y-%m-%d %H:%M").to_string();
    let window_end = (event_datetime + Duration::minutes(30))
        .format("%Y-%m-%d %H:%M").to_string();
    
    Ok(EventImpactResult {
        event_id,
        event_name,
        datetime,
        country,
        currency,
        impact,
        window_start,
        window_end,
        pair_impacts,
        observations,
    })
}
