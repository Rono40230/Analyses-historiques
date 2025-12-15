mod db_helpers;
mod file_processor;

use crate::services::create_cleaned_dir;
pub use file_processor::ImportCleanResult;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCleanReport {
    pub total_files: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<ImportCleanResult>,
}

#[tauri::command]
pub async fn import_and_clean_files(paths: Vec<String>) -> Result<ImportCleanReport, String> {
    tracing::info!(
        "📥 Import avec nettoyage automatique de {} fichiers",
        paths.len()
    );

    let mut report = ImportCleanReport {
        total_files: paths.len(),
        successful: 0,
        failed: 0,
        results: Vec::new(),
    };

    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("data")
        .join("csv");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("Erreur création répertoire: {}", e))?;
    }

    let temp_dir = create_cleaned_dir()?;

    tracing::info!("📂 Dossier de destination: {}", data_dir.display());
    tracing::info!("🧹 Dossier temporaire: {}", temp_dir.display());

    for (index, path) in paths.iter().enumerate() {
        tracing::info!("[{}/{}] Traitement: {}", index + 1, paths.len(), path);

        let result = file_processor::process_single_file(path, &temp_dir, &data_dir);

        match &result.import_status as &str {
            "success" => report.successful += 1,
            _ => report.failed += 1,
        }

        report.results.push(result);
    }

    tracing::info!(
        "📊 Import terminé: {} succès, {} échecs",
        report.successful,
        report.failed
    );

    Ok(report)
}
