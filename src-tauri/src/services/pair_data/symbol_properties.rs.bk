// services/pair_data/symbol_properties.rs
// Gestion des propriétés des symboles (valeur du point, pip, etc.)

use crate::models::AssetProperties;

/// Retourne la valeur d'un point (Tick Size) pour un symbole donné
/// C'est la plus petite variation de prix possible (ou l'unité de base pour les calculs)
pub fn get_point_value(symbol: &str) -> f64 {
    let props = AssetProperties::from_symbol(symbol);
    
    // Pour le Forex, le "point" MT5 est 1/10 de pip
    if props.unit == "pips" {
        return props.pip_value / 10.0;
    }
    
    // Pour les indices/crypto, point = pip = 1.0
    props.pip_value
}

/// Retourne la valeur d'un Pip standard
/// - Forex: 10 points
/// - Indices/Crypto: 1 point (souvent)
#[allow(dead_code)]
pub fn get_pip_value(symbol: &str) -> f64 {
    let props = AssetProperties::from_symbol(symbol);
    props.pip_value
}

/// Normalise une valeur en pips selon le symbole
pub fn normalize_to_pips(value: f64, symbol: &str) -> f64 {
    let props = AssetProperties::from_symbol(symbol);
    props.normalize(value)
}

/// Détermine si c'est une paire Forex
#[allow(dead_code)]
fn is_forex_pair(symbol: &str) -> bool {
    let s = symbol.to_uppercase();
    // Liste non exhaustive mais couvre les majeurs
    let currencies = ["EUR", "USD", "GBP", "JPY", "CAD", "AUD", "NZD", "CHF"];

    // Si contient 2 devises de la liste
    let count = currencies.iter().filter(|c| s.contains(*c)).count();
    count >= 2
}
