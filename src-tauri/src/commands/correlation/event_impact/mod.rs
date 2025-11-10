// event_impact/mod.rs - Point d'entrée du module event_impact
// Exporte la commande Tauri et les types publics

mod types;
mod helpers;

pub use types::{PairImpactDetail, EventImpactResult};
pub use helpers::{get_pip_value, currency_to_country, get_available_pairs};

use rusqlite::Connection;
use chrono::{DateTime, Utc, TimeZone, Duration};
use tauri::State;
use crate::commands::candle_index_commands::CandleIndexState;
use crate::commands::pair_data_commands::PairDataState;
use crate::services::DatabaseLoader;
use super::volatility_helpers::{parse_sqlite_datetime, calculate_volatilities_optimized};

#[tauri::command]
pub async fn get_event_impact_by_pair(
    event_type: String,
    event_count: i32,
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
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Récupérer TOUTES les occurrences de cet événement depuis 2024-01-01
    let mut event_stmt = conn
        .prepare("SELECT id, datetime(event_time), symbol, impact FROM calendar_events WHERE description = ?1 AND event_time >= '2024-01-01' ORDER BY event_time")
        .map_err(|e| format!("Failed to prepare event query: {}", e))?;
    
    let events: Vec<(i32, String, String, String)> = event_stmt
        .query_map([&event_type], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| format!("Failed to query events: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect events: {}", e))?;
    
    if events.is_empty() {
        return Err(format!("No events found for type: {}", event_type));
    }
    
    // Récupérer info de la première occurrence
    let (first_id, first_datetime, currency, impact) = &events[0];
    let country = currency_to_country(currency);
    
    let first_event_datetime = parse_sqlite_datetime(first_datetime)?;
    let window_start = first_event_datetime.format("%Y-%m-%d %H:%M").to_string();
    let window_end = (first_event_datetime + Duration::minutes(120)).format("%Y-%m-%d %H:%M").to_string();
    
    let (_, last_datetime, _, _) = &events[events.len() - 1];
    let _last_event_datetime = parse_sqlite_datetime(last_datetime)?;
    let last_datetime_formatted = last_datetime.clone();
    
    // Créer le DatabaseLoader depuis le pair_data pool
    let pair_pool = pair_state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pair pool: {}", e))?
        .clone()
        .ok_or("Pair database pool not initialized")?;
    
    let db_loader = DatabaseLoader::new(pair_pool);
    
    // Créer un CandleIndex qui utilise le DatabaseLoader pour charger les paires
    let mut candle_index = crate::services::candle_index::CandleIndex::with_db_loader(db_loader.clone());
    
    // Obtenir toutes les paires disponibles depuis la BD
    let pairs = get_available_pairs(&db_loader)?;
    
    // Préparer les datetimes des événements
    let event_datetimes: Result<Vec<DateTime<Utc>>, String> = events
        .iter()
        .map(|(_, datetime_str, _, _)| {
            let naive_dt = parse_sqlite_datetime(datetime_str)?;
            Ok(Utc.from_utc_datetime(&naive_dt))
        })
        .collect();
    let event_datetimes = event_datetimes?;
    
    // Charger les paires à la demande
    for pair in &pairs {
        candle_index.load_pair_candles(pair)?;
    }
    
    let pair_impacts = calculate_pair_impacts(&pairs, &event_datetimes[0], &candle_index)?;
    let observations = generate_observations(&pair_impacts);
    
    Ok(EventImpactResult {
        event_id: *first_id,
        event_name: event_type.clone(),
        datetime: first_datetime.clone(),
        last_datetime: last_datetime_formatted,
        country,
        currency: currency.clone(),
        impact: impact.clone(),
        event_count,
        window_start,
        window_end,
        pair_impacts,
        observations,
    })
}

/// Calcule l'impact pour chaque paire
fn calculate_pair_impacts(
    pairs: &[String],
    event_datetime: &DateTime<Utc>,
    candle_index: &crate::services::candle_index::CandleIndex,
) -> Result<Vec<PairImpactDetail>, String> {
    let mut pair_impacts = Vec::new();
    
    for pair in pairs {
        let metrics = calculate_volatilities_optimized(
            candle_index,
            pair,
            event_datetime.naive_utc(),
            30,  // event_window_minutes
            7,   // baseline_days_back
            get_pip_value(pair),
        )?;
        
        let event_volatility = metrics.event_volatility;
        let baseline_volatility = metrics.baseline_volatility;
        let multiplier = if baseline_volatility > 0.0 {
            event_volatility / baseline_volatility
        } else {
            0.0
        };
        
        let direction = if multiplier > 10.0 {
            "HAUSSIER".to_string()
        } else if multiplier > 5.0 {
            "BAISSIER".to_string()
        } else {
            "NEUTRE".to_string()
        };
        
        let points = event_volatility / 10.0;
        let pip_value = get_pip_value(pair);
        let price = points * pip_value;
        
        pair_impacts.push(PairImpactDetail {
            symbol: pair.clone(),
            event_volatility,
            baseline_volatility,
            event_volatility_formatted: format!("{:.1}", event_volatility),
            baseline_volatility_formatted: format!("{:.1}", baseline_volatility),
            points,
            points_formatted: format!("{:.1}", points),
            price,
            price_formatted: format!("{:.2}", price),
            multiplier,
            direction,
        });
    }
    
    // Trier par multiplicateur décroissant
    pair_impacts.sort_by(|a, b| {
        b.multiplier.partial_cmp(&a.multiplier)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    Ok(pair_impacts)
}

/// Génère des observations basées sur les données
fn generate_observations(pair_impacts: &[PairImpactDetail]) -> Vec<String> {
    let mut observations = Vec::new();
    
    // Observation 1: Impact le plus fort
    if let Some(top_pair) = pair_impacts.first() {
        observations.push(format!(
            "{} a enregistré le plus fort impact avec {:.0} pips, soit {:.1}× sa volatilité normale",
            top_pair.symbol, top_pair.event_volatility, top_pair.multiplier
        ));
    }
    
    // Observation 2: Paire avec la plus grosse variation
    if let Some(biggest_vol) = pair_impacts.iter().max_by(|a, b| 
        a.event_volatility.partial_cmp(&b.event_volatility)
            .unwrap_or(std::cmp::Ordering::Equal)
    ) {
        observations.push(format!(
            "Variation maximale observée: {} avec {:.1} pips de volatilité événement",
            biggest_vol.symbol, biggest_vol.event_volatility
        ));
    }
    
    // Observation 3: Paires réactives et conseil
    let high_impact_count = pair_impacts.iter().filter(|p| p.multiplier > 5.0).count();
    if high_impact_count > 0 {
        let avg_multiplier = pair_impacts.iter()
            .filter(|p| p.multiplier > 5.0)
            .map(|p| p.multiplier)
            .sum::<f64>() / high_impact_count.max(1) as f64;
        
        observations.push(format!(
            "⚠️ Attention: {} paires ont montré une volatilité EXCESSIVE (multiplicateur >5×). Ces multiplicateurs élevés (moy. {:.1}×) indiquent une réaction disproportionnée. À éviter en trading régulier, risque trop élevé pour le gain potentiel",
            high_impact_count, avg_multiplier
        ));
    }
    
    observations
}
