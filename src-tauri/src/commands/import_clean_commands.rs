// commands/import_clean_commands.rs - Import avec nettoyage automatique
// Commande unifiée qui nettoie ET importe en une seule opération

use crate::services::{create_cleaned_dir, process_file_with_cleaning, ProcessResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Rapport combiné de nettoyage + import
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCleanReport {
    pub total_files: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<ImportCleanResult>,
}

/// Résultat pour un fichier individuel
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCleanResult {
    pub original_file: String,
    pub import_status: String,
    pub lines_imported: usize,
    pub cleaning_stats: Option<FileCleaningStats>,
    pub error_message: Option<String>,
}

/// Statistiques de nettoyage pour un fichier
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCleaningStats {
    pub lines_processed: usize,
    pub lines_cleaned: usize,
    pub errors: usize,
    pub warnings: Vec<String>,
}

/// Commande Tauri : Nettoie ET importe des fichiers CSV en une seule opération
#[tauri::command]
pub async fn import_and_clean_files(paths: Vec<String>) -> Result<ImportCleanReport, String> {
    println!("📥 Import avec nettoyage automatique de {} fichiers", paths.len());
    
    let mut report = ImportCleanReport {
        total_files: paths.len(),
        successful: 0,
        failed: 0,
        results: Vec::new(),
    };
    
    // Créer le dossier de destination pour les fichiers importés
    let data_dir = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?
        .join("volatility-analyzer")
        .join("data")
        .join("csv");
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Erreur création répertoire: {}", e))?;
    }
    
    // Créer un dossier temporaire pour les fichiers nettoyés
    let temp_dir = create_cleaned_dir()?;
    
    println!("📂 Dossier de destination: {}", data_dir.display());
    println!("🧹 Dossier temporaire: {}", temp_dir.display());
    
    // Traiter chaque fichier
    for (index, path) in paths.iter().enumerate() {
        println!("\n[{}/{}] Traitement: {}", index + 1, paths.len(), path);
        
        let result = process_single_file(path, &temp_dir, &data_dir);
        
        match &result.import_status as &str {
            "success" => report.successful += 1,
            _ => report.failed += 1,
        }
        
        report.results.push(result);
    }
    
    println!("\n📊 Import terminé: {} succès, {} échecs", report.successful, report.failed);
    
    Ok(report)
}

/// Traite un fichier individuel et retourne un résultat structuré
fn process_single_file(
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
        Ok(ProcessResult { pair, timeframe, lines_cleaned, errors, error_rate }) => {
            println!("✅ Fichier importé avec succès: {} ({})", pair, timeframe);
            
            ImportCleanResult {
                original_file: file_name,
                import_status: if error_rate >= 1.0 { "partial".to_string() } else { "success".to_string() },
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
            eprintln!("❌ Erreur: {}", e);
            
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

