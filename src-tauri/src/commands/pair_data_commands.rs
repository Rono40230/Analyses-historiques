use std::path::Path;
use std::fs;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::services::PairDataConverter;
use crate::db::DbPool;
use chrono::Utc;
use tracing::{info, error};

/// État Tauri pour la DB paires (stockage des données de trading)
pub struct PairDataState {
    #[allow(dead_code)]
    pub pool: Mutex<Option<DbPool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportSummary {
    pub total_files: usize,
    pub successful: usize,
    pub failed: usize,
    pub pairs_updated: Vec<String>,
    pub timeframes: Vec<String>,
    pub errors: Vec<String>,
}

/// Commande d'import multi-fichiers de données de paires
/// NOUVEAU: Au lieu de sauvegarder CSV, insère directement dans pairs.db
#[tauri::command]
pub async fn import_pair_data(
    state: tauri::State<'_, PairDataState>,
    paths: Vec<String>,
) -> Result<ImportSummary, String> {
    info!("📥 Import de {} fichiers de paires vers BD", paths.len());
    
    let mut summary = ImportSummary {
        total_files: paths.len(),
        successful: 0,
        failed: 0,
        pairs_updated: Vec::new(),
        timeframes: Vec::new(),
        errors: Vec::new(),
    };
    
    // Obtenir le pool de la DB paires
    let pool = {
        let pool_opt = state.pool.lock().unwrap();
        pool_opt.clone().ok_or("DB pool not initialized")?
    };
    
    for path in paths {
        match process_single_file(&path, &pool) {
            Ok((pair, timeframe, row_count)) => {
                summary.successful += 1;
                
                if !summary.pairs_updated.contains(&pair) {
                    summary.pairs_updated.push(pair);
                }
                
                if !summary.timeframes.contains(&timeframe) {
                    summary.timeframes.push(timeframe);
                }
                
                info!("✅ Fichier importé: {} ({} lignes)", path, row_count);
            }
            Err(e) => {
                summary.failed += 1;
                let file_name = Path::new(&path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let error_msg = format!("{}: {}", file_name, e);
                summary.errors.push(error_msg);
                error!("❌ Erreur import {}: {}", path, e);
            }
        }
    }
    
    info!("📊 Import terminé: {} succès, {} échecs", summary.successful, summary.failed);
    
    Ok(summary)
}

/// Traite un fichier individuel: parse CSV → INSERT en DB → supprime CSV source
fn process_single_file(
    source_path: &str,
    _pool: &DbPool,
) -> Result<(String, String, usize), String> {
    // 1. Lire et normaliser le CSV
    info!("🔄 Normalisation: {}", source_path);
    let candles = PairDataConverter::read_and_normalize(source_path)?;
    
    if candles.is_empty() {
        return Err("Aucune donnée valide trouvée".to_string());
    }
    
    let row_count = candles.len();
    
    // 2. Extraire les métadonnées
    let filename = Path::new(source_path)
        .file_name()
        .ok_or("Nom de fichier invalide")?
        .to_str()
        .ok_or("Nom de fichier non-UTF8")?;
    
    info!("📊 Extraction métadonnées de: {}", filename);
    let metadata = PairDataConverter::extract_metadata(&candles, filename)?;
    
    info!("   Paire: {}", metadata.pair);
    info!("   Timeframe: {}", metadata.timeframe);
    info!("   Période: {} → {} ({} candles)", metadata.start_date, metadata.end_date, row_count);
    
    // 3. Ouvrir une connexion rusqlite directe au fichier pairs.db
    let db_path = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("pairs.db");
    
    let mut conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open pairs.db: {}", e))?;
    
    let imported_at = Utc::now().to_rfc3339();
    
    // 4. INSERT les candles en BD (bulk insert pour performance)
    info!("💾 Insertion en BD: {}/{} ({} lignes)", metadata.pair, metadata.timeframe, row_count);
    
    let tx = conn
        .transaction()
        .map_err(|e| format!("Transaction begin error: {}", e))?;
    
    // Préparer le statement une fois au lieu de pour chaque ligne
    let mut stmt = tx
        .prepare(
            "INSERT INTO candle_data (symbol, timeframe, time, open, high, low, close, volume, imported_at, source_file)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .map_err(|e| format!("Prepare error: {}", e))?;
    
    for candle in &candles {
        // Convertir timestamp Unix en DateTime RFC3339
        let dt = chrono::DateTime::<Utc>::from_timestamp(candle.timestamp, 0)
            .ok_or(format!("Invalid timestamp: {}", candle.timestamp))?;
        let time_str = dt.to_rfc3339();
        
        stmt.execute(rusqlite::params![
            &metadata.pair,
            &metadata.timeframe,
            &time_str,
            candle.open,
            candle.high,
            candle.low,
            candle.close,
            candle.volume,
            &imported_at,
            filename,
        ])
        .map_err(|e| format!("INSERT candle_data error: {}", e))?;
    }
    
    drop(stmt); // Libérer le statement avant de continuer
    
    info!("✅ {} candles insérés en BD", row_count);
    
    // 5. Mettre à jour pair_metadata
    tx.execute(
        "INSERT INTO pair_metadata (symbol, timeframe, row_count, last_updated, last_imported_file)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(symbol, timeframe) DO UPDATE SET
            row_count = row_count + excluded.row_count,
            last_updated = excluded.last_updated,
            last_imported_file = excluded.last_imported_file",
        rusqlite::params![
            &metadata.pair,
            &metadata.timeframe,
            row_count as i32,
            &imported_at,
            filename,
        ]
    )
    .map_err(|e| format!("UPDATE pair_metadata error: {}", e))?;
    
    info!("✅ Métadonnées paire mises à jour");
    
    // 6. Logger l'import
    tx.execute(
        "INSERT INTO import_log (filename, symbol, timeframe, row_count, expected_row_count, status, imported_at)
         VALUES (?, ?, ?, ?, ?, 'success', ?)",
        rusqlite::params![
            filename,
            &metadata.pair,
            &metadata.timeframe,
            row_count as i32,
            row_count as i32,
            &imported_at,
        ]
    )
    .map_err(|e| format!("INSERT import_log error: {}", e))?;
    
    info!("✅ Import loggé");
    
    // Commit transaction
    tx.commit()
        .map_err(|e| format!("Transaction commit error: {}", e))?;
    
    // 7. Supprimer le fichier source
    info!("🗑️  Suppression source: {}", source_path);
    fs::remove_file(source_path)
        .map_err(|e| format!("Erreur suppression fichier source: {}", e))?;
    
    info!("✅ Fichier source supprimé");
    
    Ok((metadata.pair, metadata.timeframe, row_count))
}
