// commands/candle_index_commands.rs
// Gère le chargement et l'initialisation du CandleIndex global
// Appelé une fois au démarrage de l'app

use std::sync::Mutex;
use tauri::State;
use crate::services::candle_index::CandleIndex;

pub struct CandleIndexState {
    pub index: Mutex<Option<CandleIndex>>,
}

/// Initialise l'index des candles au démarrage (LAZY LOADING)
/// Crée un index vide et charge les paires seulement à la demande
/// Cela rend l'app responsiv au démarrage (< 1s au lieu de 50s)
#[tauri::command]
pub async fn init_candle_index(
    state: State<'_, CandleIndexState>,
) -> Result<String, String> {
    let index = CandleIndex::new_lazy()?;
    
    let mut index_state = state.index.lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;
    
    *index_state = Some(index);
    
    Ok("CandleIndex initialized (lazy loading mode) - paires chargées à la demande".to_string())
}

/// Charge une paire spécifique dans l'index (à la demande)
/// Appelé automatiquement quand l'utilisateur sélectionne une paire
#[tauri::command]
pub async fn load_pair_candles(
    symbol: String,
    state: State<'_, CandleIndexState>,
) -> Result<String, String> {
    let mut index_state = state.index.lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;
    
    let index = index_state.as_mut()
        .ok_or("CandleIndex not initialized. Call init_candle_index first.")?;
    
    match index.load_pair_candles(&symbol)? {
        true => Ok(format!("Paire {} chargée avec succès", symbol)),
        false => Ok(format!("Paire {} déjà en cache", symbol)),
    }
}

/// Retourne les stats de l'index (pour UI/debugging)
#[tauri::command]
pub async fn get_candle_index_stats(
    state: State<'_, CandleIndexState>,
) -> Result<String, String> {
    let index_state = state.index.lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;
    
    match &*index_state {
        Some(index) => {
            let stats = index.get_stats();
            let pairs: Vec<String> = index.get_available_pairs();
            Ok(format!("Pairs: {:?}, Stats: {:?}", pairs, stats))
        }
        None => Err("CandleIndex not initialized. Call init_candle_index first.".to_string())
    }
}
