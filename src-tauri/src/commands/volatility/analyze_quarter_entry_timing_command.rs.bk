// commands/volatility/analyze_quarter_entry_timing_command.rs
// Commande pour analyser le meilleur moment d'entrée dans un quarter spécifique
// Teste chaque minute du quarter sur tout l'historique et retourne l'offset optimal

mod analyze_quarter_entry_timing_helpers;

use serde::{Deserialize, Serialize};
use tauri::command;
use analyze_quarter_entry_timing_helpers::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarterEntryTimingResponse {
    pub symbol: String,
    pub hour: u8,
    pub quarter: u8,
    pub optimal_offset_minutes: u8,  // 0-14 minutes dans le quarter
    pub optimal_win_rate: f64,       // 0-1
    pub total_occurrences_analyzed: usize,
    pub confidence_score: f64,       // 0-100: comment confident on est dans ce résultat
}

#[command]
pub async fn analyze_quarter_entry_timing(
    symbol: String,
    hour: u8,
    quarter: u8,
) -> Result<QuarterEntryTimingResponse, String> {
    use crate::db;
    use crate::services::candle_index::CandleIndex;
    use crate::services::database_loader::DatabaseLoader;

    tracing::info!(
        "🎯 Analyse timing entrée: {} {}:{}* (quarter={})",
        symbol,
        hour,
        quarter * 15,
        quarter
    );

    // Créer le pool de connexions pour la BD paires
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| "Failed to get data directory".to_string())?;
    let pairs_db_path = data_dir.join("volatility-analyzer").join("pairs.db");
    let pairs_db_url = format!("sqlite://{}", pairs_db_path.display());

    let pairs_pool = db::create_pool(&pairs_db_url)
        .map_err(|e| format!("Failed to create pairs DB pool: {}", e))?;

    // Créer un CandleIndex avec DatabaseLoader
    let db_loader = DatabaseLoader::new(pairs_pool);
    let mut candle_index = CandleIndex::with_db_loader(db_loader);

    // Charger la paire
    candle_index
        .load_pair_candles(&symbol)
        .map_err(|e| format!("Failed to load pair {}: {}", symbol, e))?;

    // Récupérer tous les candles du quarter (toutes les occurrences historiques)
    let start_minute = quarter as u32 * 15;
    let end_minute = start_minute + 15;

    let all_quarter_candles = candle_index.get_candles_for_slice_all_history(
        &symbol,
        hour as u32,
        start_minute,
        end_minute,
    );

    if all_quarter_candles.is_empty() {
        return Err(format!(
            "Aucun candle trouvé pour {}:{:02} (quarter {})",
            hour,
            start_minute,
            quarter
        ));
    }

    tracing::info!(
        "📊 Candles chargées: {} pour {}:{}* (quarter={})",
        all_quarter_candles.len(),
        hour,
        start_minute,
        quarter
    );

    // Grouper les candles par jour (chaque occurence du quarter)
    let daily_occurrences = group_candles_by_day(&all_quarter_candles);

    tracing::info!(
        "📅 Occurrences trouvées: {} jours différents",
        daily_occurrences.len()
    );

    // Pour chaque jour, trouver le meilleur minute (offset 0-14)
    let mut offsets: Vec<u8> = Vec::new();
    let mut total_win_rates: f64 = 0.0;

    for daily_candles in &daily_occurrences {
        if daily_candles.is_empty() {
            continue;
        }

        // Scorer chaque minute du quarter pour ce jour
        let best_minute = find_best_minute_in_quarter(daily_candles)?;
        offsets.push(best_minute);
        
        // Calculer win-rate pour ce minute
        let win_rate = estimate_win_rate_for_minute(daily_candles, best_minute as usize)?;
        total_win_rates += win_rate;
    }

    if offsets.is_empty() {
        return Err("Aucun offset valide trouvé".to_string());
    }

    // Calculer la moyenne des offsets
    let avg_offset = (offsets.iter().map(|&o| o as f64).sum::<f64>() / offsets.len() as f64).round() as u8;

    // Borner l'offset: [0, 15)
    let optimal_offset = std::cmp::min(avg_offset, 14);

    // Calculer le win-rate moyen
    let avg_win_rate = total_win_rates / offsets.len() as f64;

    // Calculer la confiance (basée sur la consistance des offsets)
    let confidence = calculate_confidence(&offsets, optimal_offset);

    tracing::info!(
        "✅ Timing analysé: offset={}min, win_rate={:.1}%, confidence={:.0}%",
        optimal_offset,
        avg_win_rate * 100.0,
        confidence
    );

    Ok(QuarterEntryTimingResponse {
        symbol,
        hour,
        quarter,
        optimal_offset_minutes: optimal_offset,
        optimal_win_rate: avg_win_rate,
        total_occurrences_analyzed: offsets.len(),
        confidence_score: confidence,
    })
}

/// Groupe les candles par jour (pour isoler chaque occurrence du quarter)
fn group_candles_by_day(candles: &[crate::models::Candle]) -> Vec<Vec<crate::models::Candle>> {
    analyze_quarter_entry_timing_helpers::group_candles_by_day(candles)
}

/// Trouve la meilleure minute (offset 0-14) dans un jour donné pour ce quarter
fn find_best_minute_in_quarter(
    daily_candles: &[crate::models::Candle],
) -> Result<u8, String> {
    analyze_quarter_entry_timing_helpers::find_best_minute_in_quarter(daily_candles)
}

/// Estime le win-rate pour une minute spécifique
fn estimate_win_rate_for_minute(
    daily_candles: &[crate::models::Candle],
) -> Result<f64, String> {
    analyze_quarter_entry_timing_helpers::estimate_win_rate_for_minute(daily_candles)
}

/// Calcule le score de confiance (basé sur la consistance des offsets)
fn calculate_confidence(offsets: &[u8], optimal: u8) -> f64 {
    analyze_quarter_entry_timing_helpers::calculate_confidence(offsets, optimal)
}
