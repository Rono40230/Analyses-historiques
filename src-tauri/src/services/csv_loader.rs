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
    pub fn new() -> Self {
        // Utilise le dossier de données utilisateur pour éviter le hot-reload
        // ~/.local/share/volatility-analyzer/data/csv/
        let csv_directory = dirs::data_local_dir()
            .map(|dir| dir.join("volatility-analyzer").join("data").join("csv"))
            .unwrap_or_else(|| {
                // Fallback sur l'ancien chemin si le dossier utilisateur n'est pas accessible
                if cfg!(debug_assertions) {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .parent()
                        .unwrap_or(&PathBuf::from("."))
                        .join("data/csv")
                } else {
                    std::env::current_exe()
                        .ok()
                        .and_then(|exe| exe.parent().map(|p| p.join("data/csv")))
                        .unwrap_or_else(|| PathBuf::from("data/csv"))
                }
            });
        
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
                VolatilityError::CsvLoadError(format!(
                    "Cannot create directory {:?}: {}",
                    self.csv_directory, e
                ))
            })?;
            // Retourner une liste vide si le dossier vient d'être créé
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.csv_directory).map_err(|e| {
            VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e))
        })?;

        let mut symbols = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| {
                VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e))
            })?;

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
        self.parse_csv_file(&csv_file)
    }

    /// Trouve le fichier CSV correspondant à un symbole
    fn find_csv_file(&self, symbol: &str) -> Result<PathBuf> {
        let entries = fs::read_dir(&self.csv_directory).map_err(|e| {
            VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e))
            })?;

            let path = entry.path();

            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if filename.starts_with(symbol) && filename.ends_with(".csv") {
                    debug!("Found CSV file for {}: {:?}", symbol, path);
                    return Ok(path);
                }
            }
        }

        Err(VolatilityError::SymbolNotFound(format!(
            "No CSV file found for symbol: {}",
            symbol
        )))
    }

    /// Parse un fichier CSV et retourne les bougies
    fn parse_csv_file(&self, path: &Path) -> Result<Vec<Candle>> {
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

            // Parse chaque champ
            // Format: Time (UTC),Open,High,Low,Close,Volume
            if record.len() < 6 {
                warn!("Line {}: Skipping incomplete record", line_number);
                continue;
            }

            let time_str = &record[0];
            let open_str = &record[1];
            let high_str = &record[2];
            let low_str = &record[3];
            let close_str = &record[4];
            let volume_str = &record[5];

            // Parse datetime (format: "2025.01.01 00:00:00")
            let datetime = parse_datetime(time_str).map_err(|e| {
                VolatilityError::InvalidCsvData(format!("Line {}: Invalid datetime '{}': {}", line_number, time_str, e))
            })?;

            // Parse prices
            let open = parse_price(open_str, "open", line_number)?;
            let high = parse_price(high_str, "high", line_number)?;
            let low = parse_price(low_str, "low", line_number)?;
            let close = parse_price(close_str, "close", line_number)?;
            let volume = parse_price(volume_str, "volume", line_number)?;

            // Validation basique
            if high < low {
                return Err(VolatilityError::InvalidCsvData(format!(
                    "Line {}: high ({}) < low ({})",
                    line_number, high, low
                )));
            }

            // Crée la bougie avec validation
            let candle = Candle::new(datetime, open, high, low, close, volume)
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

/// Parse une date au format "YYYY.MM.DD HH:MM:SS"
fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    // Format: "2025.01.01 00:00:00"
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
