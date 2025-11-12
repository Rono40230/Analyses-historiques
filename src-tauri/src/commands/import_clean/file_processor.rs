use super::db_helpers::{insert_candles_to_db, insert_pair_metadata};
use crate::services::process_file_with_cleaning;
use crate::services::ProcessResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCleaningStats {
    pub lines_processed: usize,
    pub lines_cleaned: usize,
    pub errors: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCleanResult {
    pub original_file: String,
    pub import_status: String,
    pub lines_imported: usize,
    pub cleaning_stats: Option<FileCleaningStats>,
    pub error_message: Option<String>,
}

pub fn process_single_file(
    source_path: &str,
    temp_dir: &Path,
    data_dir: &Path,
) -> ImportCleanResult {
    let file_name = Path::new(source_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    match process_file_with_cleaning(source_path, temp_dir, data_dir) {
        Ok(ProcessResult {
            pair,
            timeframe,
            lines_cleaned,
            errors,
            error_rate,
            cleaned_file_path,
        }) => {
            tracing::info!("✅ Fichier importé avec succès: {} ({})", pair, timeframe);

            if let Err(e) = insert_candles_to_db(&cleaned_file_path, &pair, &timeframe, &file_name)
            {
                tracing::warn!("⚠️  Erreur insertion candles: {}", e);
            }

            if let Err(e) =
                insert_pair_metadata(&pair, &timeframe, lines_cleaned as i32, &file_name)
            {
                tracing::warn!("⚠️  Erreur insertion métadonnées: {}", e);
            }

            if let Err(e) = fs::remove_file(&cleaned_file_path) {
                tracing::warn!("  ⚠️  Impossible de supprimer le fichier temporaire: {}", e);
            }

            ImportCleanResult {
                original_file: file_name,
                import_status: if error_rate >= 1.0 {
                    "partial".to_string()
                } else {
                    "success".to_string()
                },
                lines_imported: lines_cleaned,
                cleaning_stats: Some(FileCleaningStats {
                    lines_processed: lines_cleaned + errors,
                    lines_cleaned,
                    errors,
                    warnings: Vec::new(),
                }),
                error_message: None,
            }
        }
        Err(e) => {
            tracing::error!("❌ Erreur: {}", e);

            ImportCleanResult {
                original_file: file_name,
                import_status: "failed".to_string(),
                lines_imported: 0,
                cleaning_stats: None,
                error_message: Some(e),
            }
        }
    }
}
