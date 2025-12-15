use super::date_parser::extract_dates_from_filename;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct PairMetadataInfo {
    pub symbol: String,
    pub timeframe: String,
    pub row_count: i32,
    pub last_updated: String,
    pub last_imported_file: String,
    pub quality_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairMetadataUI {
    pub id: i32,
    pub symbol: String,
    pub timeframe: String,
    pub candle_count: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[tauri::command]
pub async fn get_pair_metadata_from_db(
    _pair_state: State<'_, super::super::pair_data::PairDataState>,
) -> Result<Vec<PairMetadataInfo>, String> {
    use rusqlite::Connection;

    tracing::info!("📊 Getting pair metadata from pairs.db...");

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("pairs.db");

    let conn =
        Connection::open(&data_dir).map_err(|e| format!("Failed to open pairs.db: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT symbol, timeframe, row_count, last_updated, last_imported_file, data_quality_score FROM pair_metadata ORDER BY symbol, timeframe")
        .map_err(|e| format!("Query failed: {}", e))?;

    let pairs = stmt
        .query_map([], |row| {
            Ok(PairMetadataInfo {
                symbol: row.get(0)?,
                timeframe: row.get(1)?,
                row_count: row.get(2)?,
                last_updated: row.get(3)?,
                last_imported_file: row.get(4)?,
                quality_score: row.get(5)?,
            })
        })
        .map_err(|e| format!("Query execution failed: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Result collection failed: {}", e))?;

    tracing::info!("✅ Found {} pairs in database", pairs.len());
    Ok(pairs)
}

#[tauri::command]
pub async fn get_pairs_metadata(
    pair_state: State<'_, super::super::pair_data::PairDataState>,
) -> Result<Vec<PairMetadataUI>, String> {
    let pairs = get_pair_metadata_from_db(pair_state).await?;

    let mut pair_list: Vec<PairMetadataUI> = Vec::new();

    for pair in pairs {
        let candle_count: i32 = pair.row_count;
        let (start_date, end_date) = extract_dates_from_filename(&pair.last_imported_file);

        pair_list.push(PairMetadataUI {
            id: pair_list.len() as i32,
            symbol: pair.symbol,
            timeframe: pair.timeframe,
            candle_count,
            start_date,
            end_date,
        });
    }

    Ok(pair_list)
}
