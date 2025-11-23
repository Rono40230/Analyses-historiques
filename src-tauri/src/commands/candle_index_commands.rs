// commands/candle_index_commands.rs
// Gère le chargement et l'initialisation du CandleIndex global
// Appelé une fois au démarrage de l'app

use crate::commands::pair_data::PairDataState;
use crate::models::Candle;
use crate::services::candle_index::CandleIndex;
use crate::services::DatabaseLoader;
use chrono::NaiveDate;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

pub struct CandleIndexState {
    pub index: Mutex<Option<CandleIndex>>,
}

/// Initialise l'index des candles au démarrage (LAZY LOADING)
/// Crée un index vide avec DatabaseLoader pour charger paires depuis pairs.db
/// Cela rend l'app responsiv au démarrage (< 1s au lieu de 50s)
#[tauri::command]
pub async fn init_candle_index(
    state: State<'_, CandleIndexState>,
    pair_state: State<'_, PairDataState>,
) -> Result<String, String> {
    // Obtenir le pool pairs.db
    let pair_pool = pair_state
        .pool
        .lock()
        .map_err(|e| format!("Failed to lock pair pool: {}", e))?
        .clone()
        .ok_or("Pair database pool not initialized")?;

    // Créer le DatabaseLoader
    let db_loader = DatabaseLoader::new(pair_pool);

    // Créer un CandleIndex avec le DatabaseLoader
    let index = CandleIndex::with_db_loader(db_loader);

    let mut index_state = state
        .index
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    *index_state = Some(index);

    Ok("CandleIndex initialized avec DatabaseLoader - paires chargées depuis BD".to_string())
}

/// Charge une paire spécifique dans l'index (à la demande)
/// Appelé automatiquement quand l'utilisateur sélectionne une paire
#[tauri::command]
pub async fn load_pair_candles(
    symbol: String,
    state: State<'_, CandleIndexState>,
) -> Result<String, String> {
    let mut index_state = state
        .index
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    let index = index_state
        .as_mut()
        .ok_or("CandleIndex not initialized. Call init_candle_index first.")?;

    match index.load_pair_candles(&symbol)? {
        true => Ok(format!("Paire {} chargée avec succès", symbol)),
        false => Ok(format!("Paire {} déjà en cache", symbol)),
    }
}

/// Retourne les stats de l'index (pour UI/debugging)
#[tauri::command]
pub async fn get_candle_index_stats(state: State<'_, CandleIndexState>) -> Result<String, String> {
    let index_state = state
        .index
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    match &*index_state {
        Some(index) => {
            let stats = index.get_stats();
            let pairs: Vec<String> = index.get_available_pairs();
            Ok(format!("Pairs: {:?}, Stats: {:?}", pairs, stats))
        }
        None => Err("CandleIndex not initialized. Call init_candle_index first.".to_string()),
    }
}

/// Récupère les 60 candles (1min chacune) pour une heure spécifique
/// Utilisé par TÂCHE 5 pour charger les VRAIES données
#[derive(Serialize)]
pub struct CandlesForHourResponse {
    pub symbol: String,
    pub date: String,
    pub hour: u32,
    pub candle_count: usize,
    pub candles: Vec<Candle>,
}

#[tauri::command]
pub async fn get_candles_for_hour(
    symbol: String,
    date_str: String, // Format: "YYYY-MM-DD"
    hour: u32,
    state: State<'_, CandleIndexState>,
) -> Result<CandlesForHourResponse, String> {
    // Parser la date
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format '{}': {}", date_str, e))?;

    // Valider l'heure (0-23)
    if hour > 23 {
        return Err(format!("Invalid hour: {} (must be 0-23)", hour));
    }

    let index_state = state
        .index
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    let index = index_state
        .as_ref()
        .ok_or("CandleIndex not initialized. Call init_candle_index first.")?;

    // Récupérer les candles pour cette heure
    let candles = index
        .get_candles_for_hour(&symbol, date, hour)
        .ok_or(format!("No candles found for {} on {} hour {}", symbol, date_str, hour))?;

    if candles.is_empty() {
        return Err(format!(
            "No candles for {} on {} hour {} (empty result)",
            symbol, date_str, hour
        ));
    }

    Ok(CandlesForHourResponse {
        symbol,
        date: date_str,
        hour,
        candle_count: candles.len(),
        candles,
    })
}
