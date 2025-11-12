use crate::db::DbPool;
use crate::services::PairDataConverter;
use chrono::Utc;
use std::fs;
use std::path::Path;
use tracing::info;

pub fn process_single_file(
    source_path: &str,
    _pool: &DbPool,
) -> Result<(String, String, usize), String> {
    info!("🔄 Normalisation: {}", source_path);
    let candles = PairDataConverter::read_and_normalize(source_path)?;

    if candles.is_empty() {
        return Err("Aucune donnée valide trouvée".to_string());
    }

    let row_count = candles.len();

    let filename = Path::new(source_path)
        .file_name()
        .ok_or("Nom de fichier invalide")?
        .to_str()
        .ok_or("Nom de fichier non-UTF8")?;

    info!("📊 Extraction métadonnées de: {}", filename);
    let metadata = PairDataConverter::extract_metadata(&candles, filename)?;

    info!("   Paire: {}", metadata.pair);
    info!("   Timeframe: {}", metadata.timeframe);
    info!(
        "   Période: {} → {} ({} candles)",
        metadata.start_date, metadata.end_date, row_count
    );

    let db_path = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("pairs.db");

    let mut conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open pairs.db: {}", e))?;

    let imported_at = Utc::now().to_rfc3339();

    info!(
        "💾 Insertion en BD: {}/{} ({} lignes)",
        metadata.pair, metadata.timeframe, row_count
    );

    let tx = conn
        .transaction()
        .map_err(|e| format!("Transaction begin error: {}", e))?;

    let mut stmt = tx
        .prepare(
            "INSERT INTO candle_data (symbol, timeframe, time, open, high, low, close, volume, imported_at, source_file)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .map_err(|e| format!("Prepare error: {}", e))?;

    info!("📋 Prepared INSERT statement for candle_data");

    for (idx, candle) in candles.iter().enumerate() {
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
        .map_err(|e| format!("INSERT candle_data error at row {}: {}", idx, e))?;

        if idx % 50000 == 0 && idx > 0 {
            info!("  ✓ {} candles processed", idx);
        }
    }

    drop(stmt);

    info!("✅ {} candles insérés en BD", row_count);

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
        ],
    )
    .map_err(|e| format!("UPDATE pair_metadata error: {}", e))?;

    info!("✅ Métadonnées mises à jour");

    info!("📋 INSERT import_log entry");
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

    tx.commit()
        .map_err(|e| format!("Transaction commit error: {}", e))?;

    info!("🗑️  Tentative suppression: {}", source_path);
    fs::remove_file(source_path)
        .map_err(|e| format!("Erreur suppression fichier source: {}", e))?;

    info!("✅ Fichier source supprimé avec succès");
    info!(
        "🎉 Import réussi: {}/{} ({} candles)",
        metadata.pair, metadata.timeframe, row_count
    );

    Ok((metadata.pair, metadata.timeframe, row_count))
}
