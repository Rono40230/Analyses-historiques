use chrono::{DateTime, NaiveDateTime};
use crate::models::AssetProperties;

/// Parse une datetime depuis SQLite qui peut être soit un string formaté, soit un timestamp Unix
pub fn parse_sqlite_datetime(s: &str) -> Result<NaiveDateTime, String> {
    // Essayer d'abord comme timestamp Unix (nombre de secondes)
    if let Ok(timestamp) = s.parse::<i64>() {
        return DateTime::from_timestamp(timestamp, 0)
            .map(|dt| dt.naive_utc())
            .ok_or_else(|| format!("Invalid Unix timestamp: {}", s));
    }

    // Sinon, essayer les formats de date classiques
    let formats = vec![
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d",
    ];

    for format in formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, format) {
            return Ok(dt);
        }
    }

    Err(format!(
        "Cannot parse datetime from any known format: '{}'",
        s
    ))
}

/// Retourne la valeur d'1 pip pour une paire donnée
pub fn get_pip_value(symbol: &str) -> f64 {
    let props = AssetProperties::from_symbol(symbol);
    props.pip_value
}
