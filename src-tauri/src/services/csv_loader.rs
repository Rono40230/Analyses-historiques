// services/csv_loader.rs - Chargement et parsing des fichiers CSV
// Niveau 3 : Service qui lit les CSV et retourne Vec<Candle>
// Conforme .clinerules : < 300 lignes, pas d'unwrap, thiserror

use crate::models::{Candle, Result, VolatilityError};
use chrono::{DateTime, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Service de chargement des fichiers CSV
pub struct CsvLoader {
    /// Chemin vers le dossier contenant les CSV
    csv_directory: PathBuf,
}

impl CsvLoader {
    /// Crée un nouveau CSV Loader
    /// Priorité : .local/share (données utilisateur) puis data/csv (exemples)
    pub fn new() -> Self {
        let user_csv = dirs::data_local_dir()
            .map(|dir| dir.join("volatility-analyzer").join("data").join("csv"));

        let project_csv = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .join("data/csv");

        // Utiliser .local/share si il contient des CSV, sinon fallback sur data/csv
        let csv_directory = if let Some(ref user_dir) = user_csv {
            let has_user_csv = user_dir.read_dir().ok().and_then(|mut d| d.next()).is_some();
            let has_project_csv = project_csv.read_dir().ok().and_then(|mut d| d.next()).is_some();
            
            if has_user_csv {
                info!("Using user CSV: {:?}", user_dir);
                user_dir.clone()
            } else if has_project_csv {
                info!("User empty, using project CSV: {:?}", project_csv);
                project_csv
            } else {
                info!("No CSV found, using: {:?}", user_dir);
                user_dir.clone()
            }
        } else {
            project_csv
        };
        
        Self { csv_directory }
    }

    /// Crée un loader avec répertoire personnalisé
    #[allow(dead_code)]
    pub fn with_directory<P: AsRef<Path>>(path: P) -> Self {
        Self {
            csv_directory: path.as_ref().to_path_buf(),
        }
    }

    /// Liste tous les symboles disponibles dans le dossier CSV
    pub fn list_available_symbols(&self) -> Result<Vec<String>> {
        info!("Scanning CSV directory: {:?}", self.csv_directory);

        // Créer le dossier s'il n'existe pas
        if !self.csv_directory.exists() {
            info!("Creating CSV directory: {:?}", self.csv_directory);
            fs::create_dir_all(&self.csv_directory).map_err(|e| {
                VolatilityError::CsvLoadError(format!("Cannot create directory {:?}: {}", self.csv_directory, e))
            })?;
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.csv_directory)
            .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e)))?;

        let mut symbols = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e)))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    // Extrait le symbole du nom de fichier
                    // Format: BTCUSD_1 Min_Bid_2025.01.01_2025.10.30.csv
                    // On prend juste la première partie avant le premier "_"
                    if let Some(symbol) = filename.split('_').next() {
                        if !symbols.contains(&symbol.to_string()) {
                            symbols.push(symbol.to_string());
                        }
                    }
                }
            }
        }

        symbols.sort();
        info!("Found {} symbols: {:?}", symbols.len(), symbols);
        Ok(symbols)
    }

    /// Charge les bougies pour un symbole donné
    /// Cherche automatiquement le fichier correspondant dans le dossier
    pub fn load_candles(&self, symbol: &str) -> Result<Vec<Candle>> {
        info!("Loading candles for symbol: {}", symbol);

        // Trouve le fichier CSV correspondant au symbole
        let csv_file = self.find_csv_file(symbol)?;

        // Parse le fichier CSV
        self.parse_csv_file(&csv_file, symbol)
    }

    /// Trouve le fichier CSV correspondant à un symbole (peut être une devise ou une paire)
    fn find_csv_file(&self, symbol: &str) -> Result<PathBuf> {
        let entries = fs::read_dir(&self.csv_directory).map_err(|e| {
            VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e))
        })?;

        let mut matching_files = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| {
                VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e))
            })?;

            let path = entry.path();

            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                // Chercher des correspondances exactes d'abord (paire complète: EURUSD)
                if filename.starts_with(symbol) && filename.ends_with(".csv") {
                    debug!("Found CSV file for {}: {:?}", symbol, path);
                    return Ok(path);
                }
                
                // Sinon, chercher si le symbole est une devise (AUD, EUR, GBP, etc.)
                // et trouver une paire qui la contient (AUDUSD, AUDJPY, etc.)
                if symbol.len() <= 3 && filename.contains(symbol) && filename.ends_with(".csv") {
                    matching_files.push(path);
                }
            }
        }

        // Si on a trouvé des correspondances de devise, retourner la première
        if !matching_files.is_empty() {
            debug!("Found CSV file for currency {}: {:?}", symbol, matching_files[0]);
            return Ok(matching_files[0].clone());
        }

        Err(VolatilityError::SymbolNotFound(format!(
            "No CSV file found for symbol: {}",
            symbol
        )))
    }

    /// Parse un fichier CSV et retourne les bougies
    fn parse_csv_file(&self, path: &Path, symbol: &str) -> Result<Vec<Candle>> {
        info!("Parsing CSV file: {:?}", path);

        let file = fs::File::open(path).map_err(|e| {
            VolatilityError::CsvLoadError(format!("Cannot open file: {}", e))
        })?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file);

        let mut candles = Vec::new();
        let mut line_number = 1; // 1 = header

        for result in reader.records() {
            line_number += 1;
            let record = result.map_err(|e| {
                VolatilityError::InvalidCsvData(format!("Line {}: {}", line_number, e))
            })?;

            // Détection format: 6 champs (normal) ou 11 champs (virgules décimales EU)
            let (time_str, open_str, high_str, low_str, close_str, volume_str) = 
                if record.len() == 11 {
                    // Format EU: reconstruire les nombres (0,583 → 0.583)
                    (record[0].to_string(), 
                     format!("{}.{}", &record[1], &record[2]),
                     format!("{}.{}", &record[3], &record[4]),
                     format!("{}.{}", &record[5], &record[6]),
                     format!("{}.{}", &record[7], &record[8]),
                     format!("{}.{}", &record[9], &record[10]))
                } else if record.len() == 6 {
                    (record[0].to_string(), record[1].to_string(), record[2].to_string(),
                     record[3].to_string(), record[4].to_string(), record[5].to_string())
                } else {
                    warn!("Line {}: Unexpected {} fields, skipping", line_number, record.len());
                    continue;
                };

            let datetime = parse_datetime(&time_str).map_err(|e| {
                VolatilityError::InvalidCsvData(format!("Line {}: Invalid datetime '{}': {}", line_number, time_str, e))
            })?;

            // Parse prices
            let open = parse_price(&open_str, "open", line_number)?;
            let high = parse_price(&high_str, "high", line_number)?;
            let low = parse_price(&low_str, "low", line_number)?;
            let close = parse_price(&close_str, "close", line_number)?;
            let volume = parse_price(&volume_str, "volume", line_number)?;

            // Validation basique des prix
            if high < low {
                return Err(VolatilityError::InvalidCsvData(format!(
                    "Line {}: high ({}) < low ({}). CSV corrompu, réimportez les données.",
                    line_number, high, low
                )));
            }

            if open < 0.0 || high < 0.0 || low < 0.0 || close < 0.0 || volume < 0.0 {
                return Err(VolatilityError::InvalidCsvData(format!(
                    "Line {}: Valeurs négatives: open={}, high={}, low={}, close={}, vol={}",
                    line_number, open, high, low, close, volume
                )));
            }

            // Crée la bougie avec validation
            let candle = Candle::new(symbol.to_string(), datetime, open, high, low, close, volume)
                .map_err(|e| {
                    VolatilityError::ValidationError(format!("Line {}: {}", line_number, e))
                })?;

            candles.push(candle);
        }

        if candles.is_empty() {
            return Err(VolatilityError::InsufficientData(
                "No valid candles found in CSV file".to_string(),
            ));
        }

        info!("Successfully loaded {} candles", candles.len());
        Ok(candles)
    }
}

impl Default for CsvLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse une date depuis un timestamp Unix ou format "YYYY.MM.DD HH:MM:SS"
fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    // Essayer d'abord le format timestamp Unix (priorité car c'est le format réel des CSV)
    if let Ok(timestamp) = s.parse::<i64>() {
        return DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| VolatilityError::ChronoError(
                format!("Invalid Unix timestamp: {}", timestamp)
            ));
    }

    // Fallback: Format "2025.01.01 00:00:00"
    // On remplace les points par des tirets pour chrono
    let normalized = s.replace('.', "-");

    NaiveDateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S")
        .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
        .map_err(|e| VolatilityError::ChronoError(format!("Cannot parse datetime '{}': {}", s, e)))
}

/// Parse un prix (f64) avec gestion d'erreur
fn parse_price(s: &str, field_name: &str, line: usize) -> Result<f64> {
    s.parse::<f64>().map_err(|e| {
        VolatilityError::InvalidCsvData(format!(
            "Line {}: Invalid {} value '{}': {}",
            line, field_name, s, e
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_datetime() {
        // Test timestamp Unix
        let result = parse_datetime("1704117600");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);

        // Test format "YYYY.MM.DD HH:MM:SS"
        let result = parse_datetime("2025.01.01 00:00:00");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 0);
    }

    #[test]
    fn test_parse_price() {
        assert!(parse_price("42499.0", "open", 1).is_ok());
        assert!(parse_price("invalid", "open", 1).is_err());
    }

    #[test]
    fn test_csv_loader_creation() {
        let loader = CsvLoader::new();
        assert!(loader.csv_directory.to_str().unwrap().contains("data/csv"));
    }
}
