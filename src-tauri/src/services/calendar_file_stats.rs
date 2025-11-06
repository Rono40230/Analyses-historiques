// calendar_file_stats.rs
use std::fs;
use std::path::PathBuf;

/// Compte le nombre d'événements dans un fichier CSV calendrier
pub fn count_csv_events(path: &PathBuf) -> Option<i64> {
    match fs::read_to_string(path) {
        Ok(content) => {
            // Compter les lignes (en soustrayant 1 pour l'en-tête si présent)
            let line_count = content.lines().count();
            Some((line_count.saturating_sub(1)) as i64)
        }
        Err(_) => None,
    }
}

/// Extrait la période couverte depuis le nom du fichier calendar
/// Ex: "calendar_2007-01-01_2025-10-31.csv" → Some("2007-01-01 → 2025-10-31")
pub fn extract_calendar_date_range(filename: &str) -> Option<String> {
    // Format attendu: calendar_YYYY-MM-DD_YYYY-MM-DD.ext
    let name_without_ext = filename.split('.').next()?;
    let parts: Vec<&str> = name_without_ext.split('_').collect();
    
    if parts.len() >= 3 {
        let start_date = parts.get(1)?;
        let end_date = parts.get(2)?;
        
        // Vérifier que ce sont bien des dates (format YYYY-MM-DD)
        if start_date.len() == 10 && end_date.len() == 10 {
            return Some(format!("{} → {}", start_date, end_date));
        }
    }
    
    None
}
