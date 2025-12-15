use crate::models::{Result, VolatilityError};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Trouve le fichier CSV correspondant à un symbole
pub fn find_csv_file(csv_directory: &Path, symbol: &str) -> Result<PathBuf> {
    let entries = fs::read_dir(csv_directory)
        .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e)))?;

    let mut matching_files = Vec::new();

    for entry in entries {
        let entry = entry
            .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e)))?;

        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
            if filename.starts_with(symbol) && filename.ends_with(".csv") {
                debug!("Found CSV file for {}: {:?}", symbol, path);
                return Ok(path);
            }

            if symbol.len() <= 3 && filename.contains(symbol) && filename.ends_with(".csv") {
                matching_files.push(path);
            }
        }
    }

    if !matching_files.is_empty() {
        debug!(
            "Found CSV file for currency {}: {:?}",
            symbol, matching_files[0]
        );
        return Ok(matching_files[0].clone());
    }

    Err(VolatilityError::SymbolNotFound(format!(
        "No CSV file found for symbol: {}",
        symbol
    )))
}

/// Liste tous les symboles disponibles dans le dossier CSV
pub fn list_available_symbols(csv_directory: &Path) -> Result<Vec<String>> {
    info!("Scanning CSV directory: {:?}", csv_directory);

    if !csv_directory.exists() {
        info!("Creating CSV directory: {:?}", csv_directory);
        fs::create_dir_all(csv_directory).map_err(|e| {
            VolatilityError::CsvLoadError(format!(
                "Cannot create directory {:?}: {}",
                csv_directory, e
            ))
        })?;
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(csv_directory)
        .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read directory: {}", e)))?;

    let mut symbols = Vec::new();

    for entry in entries {
        let entry = entry
            .map_err(|e| VolatilityError::CsvLoadError(format!("Cannot read entry: {}", e)))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
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
