// services/csv_cleaner.rs - Nettoyeur de CSV européens
// Conforme .clinerules : < 150L, pas d'unwrap()

use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleaningReport {
    pub original_file: String,
    pub cleaned_file: String,
    pub status: String,
    pub lines_processed: usize,
    pub lines_cleaned: usize,
    pub errors: usize,
    pub warnings: Vec<String>,
}

/// Nettoie un fichier CSV avec format européen (virgules décimales)
pub fn clean_european_csv(input_path: &str, output_dir: &Path) -> Result<CleaningReport, String> {
    let input = Path::new(input_path);
    let filename = input.file_name().and_then(|n| n.to_str()).ok_or("Nom invalide")?;
    let output_filename = filename.replace(".csv", "_cleaned.csv");
    let output_path = output_dir.join(&output_filename);
    
    println!("🧹 Nettoyage: {} → {}", input_path, output_path.display());
    
    let input_file = File::open(input_path).map_err(|e| format!("Ouverture: {}", e))?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_path).map_err(|e| format!("Création: {}", e))?;
    
    let mut report = CleaningReport {
        original_file: filename.to_string(),
        cleaned_file: output_path.to_string_lossy().to_string(),
        status: "success".to_string(),
        lines_processed: 0, lines_cleaned: 0, errors: 0, warnings: Vec::new(),
    };
    
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => { report.errors += 1; report.warnings.push(format!("L{}: {}", line_num+1, e)); continue; }
        };
        report.lines_processed += 1;
        
        if line_num == 0 {
            writeln!(output_file, "timestamp,open,high,low,close,volume").map_err(|e| format!("Header: {}", e))?;
            continue;
        }
        
        match clean_line(&line) {
            Ok(cleaned) => { writeln!(output_file, "{}", cleaned).map_err(|e| format!("L{}: {}", line_num+1, e))?; report.lines_cleaned += 1; }
            Err(e) => { report.errors += 1; report.warnings.push(format!("L{}: {}", line_num+1, e)); }
        }
    }
    
    // Calculer taux d'erreur : < 1% = success, sinon partial
    let error_rate = if report.lines_processed > 0 {
        (report.errors as f64 / report.lines_processed as f64) * 100.0
    } else {
        0.0
    };
    
    if error_rate >= 1.0 { 
        report.status = "partial".to_string(); 
        println!("⚠️ {} lignes nettoyées ({} erreurs = {:.2}%)", report.lines_cleaned, report.errors, error_rate);
    } else if report.errors > 0 {
        println!("✅ {} lignes nettoyées ({} erreurs = {:.2}% < 1%)", report.lines_cleaned, report.errors, error_rate);
    } else {
        println!("✅ {} lignes nettoyées (0 erreur)", report.lines_cleaned);
    }
    
    Ok(report)
}

/// Nettoie une ligne individuelle
fn clean_line(line: &str) -> Result<String, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 2 { return Err("Format invalide".to_string()); }
    
    // Timestamp = première partie (peut contenir un espace)
    let timestamp_parts: Vec<&str> = parts.iter()
        .take_while(|p| p.contains('.') || p.contains(':'))
        .map(|s| *s)
        .collect();
    
    if timestamp_parts.is_empty() { return Err("Pas de timestamp".to_string()); }
    
    let timestamp = timestamp_parts.join(" ");
    let data_start = timestamp_parts.len();
    
    // Valeurs numériques = tout après le timestamp
    let values: Vec<&str> = parts[data_start..].iter().map(|s| *s).collect();
    
    // Reconstruction OHLCV : fusionner paires adjacentes si besoin
    let ohlcv = reconstruct_ohlcv(&values)?;
    
    Ok(format!("{},{},{},{},{},{}", timestamp, ohlcv[0], ohlcv[1], ohlcv[2], ohlcv[3], ohlcv[4]))
}

/// Reconstruit les 5 valeurs OHLCV depuis un tableau de valeurs séparées
fn reconstruct_ohlcv(values: &[&str]) -> Result<Vec<String>, String> {
    if values.is_empty() { return Err("Aucune valeur".to_string()); }
    
    let mut result = Vec::new();
    let mut i = 0;
    
    while result.len() < 5 && i < values.len() {
        let current = values[i];
        
        // Si nombre court (0-2 chiffres) ET suivi d'un nombre (partie décimale), fusionner
        if current.len() <= 2 && i + 1 < values.len() && values[i + 1].len() >= 1 && values[i + 1].chars().all(|c| c.is_numeric()) {
            result.push(format!("{}.{}", current, values[i + 1]));
            i += 2;
        }
        // Sinon prendre tel quel
        else {
            result.push(current.to_string());
            i += 1;
        }
    }
    
    if result.len() < 5 { return Err(format!("Seulement {} valeurs trouvées", result.len())); }
    Ok(result[..5].to_vec())
}

/// Crée le répertoire de sortie pour les fichiers nettoyés
pub fn create_cleaned_dir() -> Result<PathBuf, String> {
    let dir = dirs::data_local_dir()
        .ok_or("Impossible d'obtenir le dossier de données")?
        .join("volatility-analyzer")
        .join("cleaned");
    
    create_dir_all(&dir)
        .map_err(|e| format!("Erreur création dossier: {}", e))?;
    
    Ok(dir)
}
