// event_impact/helpers.rs - Fonctions utilitaires pour analyse d'impact
// Conforme .clinerules: <100L, fonctions pures

use crate::services::CsvLoader;

/// Retourne la valeur d'1 pip pour une paire donnée
pub fn get_pip_value(symbol: &str) -> f64 {
    match symbol {
        "ADAUSD" => 0.0001,
        "BTCUSD" => 1.00,
        "CADJPY" => 0.01,
        "CHFJPY" => 0.01,
        "ETHUSD" => 0.01,
        "GBPJPY" => 0.01,
        "LINKUSD" => 0.001,
        "LTCUSD" => 0.01,
        "UNIUSD" => 0.001,
        "USDCAD" => 0.0001,
        "USDJPY" => 0.01,
        "XAGUSD" => 0.001,
        "XAUUSD" => 0.01,
        "XLMUSD" => 0.00001,
        _ => 0.0001, // valeur par défaut
    }
}

/// Mappe une devise ISO à son pays
pub fn currency_to_country(currency: &str) -> String {
    match currency {
        "USD" => "United States",
        "EUR" => "Eurozone",
        "GBP" => "United Kingdom",
        "JPY" => "Japan",
        "CHF" => "Switzerland",
        "CAD" => "Canada",
        "AUD" => "Australia",
        "NZD" => "New Zealand",
        "CNY" => "China",
        "INR" => "India",
        "ZAR" => "South Africa",
        "MXN" => "Mexico",
        _ => "Unknown",
    }.to_string()
}

/// Récupère toutes les paires disponibles avec priorité
pub fn get_available_pairs() -> Result<Vec<String>, String> {
    let loader = CsvLoader::new();
    let symbols = loader
        .list_available_symbols()
        .map_err(|e| format!("Failed to get available symbols: {}", e))?;
    
    let priority_pairs = vec!["USDJPY", "GBPJPY", "BTCUSD", "ETHUSD", "EURUSD", "GBPUSD", 
                              "USDCAD", "USDCHF", "CADJPY", "CHFJPY", "XAUUSD", "XAGUSD",
                              "LTCCHF", "TRXUSD", "LNKUSD", "UNIUSD", "EURZAR", "NZDMXN"];
    
    let mut result = Vec::new();
    
    // Ajouter les paires prioritaires si disponibles
    for pair in priority_pairs {
        if symbols.contains(&pair.to_string()) {
            result.push(pair.to_string());
        }
    }
    
    // Ajouter les autres paires qui ne sont pas dans la liste prioritaire
    for symbol in symbols {
        if !result.contains(&symbol) {
            result.push(symbol);
        }
    }
    
    Ok(result)
}
