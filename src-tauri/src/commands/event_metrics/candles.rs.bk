use super::CandlesState;
use crate::models::Candle;
use tauri::State;
use tracing::info;

#[tauri::command]
pub async fn load_candles_for_metrics(
    candles: Vec<Candle>,
    candles_state: State<'_, CandlesState>,
) -> Result<String, String> {
    info!("Loading {} candles for metrics calculation", candles.len());

    candles_state
        .candles
        .lock()
        .map_err(|e| format!("Failed to lock candles: {}", e))?
        .clear();

    candles_state
        .candles
        .lock()
        .map_err(|e| format!("Failed to lock candles: {}", e))?
        .extend(candles);

    info!("Candles loaded successfully");
    Ok(format!(
        "Loaded {} candles",
        candles_state
            .candles
            .lock()
            .map_err(|e| format!("Failed to lock candles: {}", e))?
            .len()
    ))
}

#[tauri::command]
pub async fn get_available_symbols(
    candles_state: State<'_, CandlesState>,
) -> Result<Vec<String>, String> {
    let candles_guard = candles_state
        .candles
        .lock()
        .map_err(|e| format!("Failed to lock candles: {}", e))?;

    let mut symbols: Vec<String> = candles_guard
        .iter()
        .map(|c| c.symbol.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    symbols.sort();
    Ok(symbols)
}

#[tauri::command]
pub async fn clear_candles(candles_state: State<'_, CandlesState>) -> Result<String, String> {
    info!("Clearing candles from memory");

    candles_state
        .candles
        .lock()
        .map_err(|e| format!("Failed to lock candles: {}", e))?
        .clear();

    info!("Candles cleared successfully");
    Ok("Candles cleared".to_string())
}
