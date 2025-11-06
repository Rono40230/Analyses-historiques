// commands/csv_cleaner_commands.rs
use crate::services::{clean_european_csv, create_cleaned_dir, CleaningReport};

/// Nettoie plusieurs fichiers CSV européens
#[tauri::command]
pub async fn clean_csv_files(paths: Vec<String>) -> Result<Vec<CleaningReport>, String> {
    println!("🧹 Nettoyage de {} fichiers CSV", paths.len());
    
    let output_dir = create_cleaned_dir()?;
    println!("📂 Dossier de sortie: {}", output_dir.display());
    
    let mut reports = Vec::new();
    
    for path in paths {
        match clean_european_csv(&path, &output_dir) {
            Ok(report) => {
                println!("✅ {}: {} lignes nettoyées", report.original_file, report.lines_cleaned);
                reports.push(report);
            }
            Err(e) => {
                eprintln!("❌ Erreur nettoyage {}: {}", path, e);
                reports.push(CleaningReport {
                    original_file: path.clone(),
                    cleaned_file: String::new(),
                    status: "error".to_string(),
                    lines_processed: 0,
                    lines_cleaned: 0,
                    errors: 1,
                    warnings: vec![e],
                });
            }
        }
    }
    
    let total_cleaned: usize = reports.iter().map(|r| r.lines_cleaned).sum();
    let total_errors: usize = reports.iter().map(|r| r.errors).sum();
    
    println!("📊 Nettoyage terminé: {} lignes, {} erreurs", total_cleaned, total_errors);
    
    Ok(reports)
}
