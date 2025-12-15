mod discovery;
mod parser;

pub use discovery::{find_csv_file, list_available_symbols};
pub use parser::{parse_datetime, parse_price};

use crate::models::{Candle, Result, VolatilityError};
use csv::ReaderBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Service de chargement des fichiers CSV
pub struct CsvLoader {
    csv_directory: PathBuf,
}

impl CsvLoader {
    /// Crée un nouveau CSV Loader
    pub fn new() -> Self {
        let csv_directory = dirs::data_local_dir()
            .map(|dir| dir.join("volatility-analyzer").join("data").join("csv"))
            .unwrap_or_else(|| PathBuf::from("~/.local/share/volatility-analyzer/data/csv"));

        info!("Using user CSV directory: {:?}", csv_directory);

        Self { csv_directory }
    }

    #[allow(dead_code)]
    pub fn with_directory<P: AsRef<Path>>(path: P) -> Self {
        Self {
            csv_directory: path.as_ref().to_path_buf(),
        }
    }

    /// Liste tous les symboles disponibles
    pub fn list_available_symbols(&self) -> Result<Vec<String>> {
        list_available_symbols(&self.csv_directory)
    }

    /// Charge les bougies pour un symbole donné
    pub fn load_candles(&self, symbol: &str) -> Result<Vec<Candle>> {
        info!("Loading candles for symbol: {}", symbol);
        let csv_file = find_csv_file(&self.csv_directory, symbol)?;
        self.parse_csv_file(&csv_file, symbol)
    }

    /// Parse un fichier CSV et retourne les bougies
    fn parse_csv_file(&self, path: &Path, symbol: &str) -> Result<Vec<Candle>> {
        info!("Parsing CSV file: {:?}", path);

        let file = fs::File::open(path)
            .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot open file: {}", e)))?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file);

        let mut candles = Vec::new();
        let mut line_number = 1;

        for result in reader.records() {
            line_number += 1;
            let record = result.map_err(|e| {
                VolatilityError::InvalidCsvData(format!("Line {}: {}", line_number, e))
            })?;

            let (time_str, open_str, high_str, low_str, close_str, volume_str) =
                if record.len() == 11 {
                    (
                        record[0].to_string(),
                        format!("{}.{}", &record[1], &record[2]),
                        format!("{}.{}", &record[3], &record[4]),
                        format!("{}.{}", &record[5], &record[6]),
                        format!("{}.{}", &record[7], &record[8]),
                        format!("{}.{}", &record[9], &record[10]),
                    )
                } else if record.len() == 6 {
                    (
                        record[0].to_string(),
                        record[1].to_string(),
                        record[2].to_string(),
                        record[3].to_string(),
                        record[4].to_string(),
                        record[5].to_string(),
                    )
                } else {
                    warn!(
                        "Line {}: Unexpected {} fields, skipping",
                        line_number,
                        record.len()
                    );
                    continue;
                };

            let datetime = parse_datetime(&time_str).map_err(|e| {
                VolatilityError::InvalidCsvData(format!(
                    "Line {}: Invalid datetime '{}': {}",
                    line_number, time_str, e
                ))
            })?;

            let open = parse_price(&open_str, "open", line_number)?;
            let high = parse_price(&high_str, "high", line_number)?;
            let low = parse_price(&low_str, "low", line_number)?;
            let close = parse_price(&close_str, "close", line_number)?;
            let volume = parse_price(&volume_str, "volume", line_number)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_loader_creation() {
        let loader = CsvLoader::new();
        assert!(loader
            .csv_directory
            .to_str()
            .expect("Failed to convert path")
            .contains("data/csv"));
    }
}
