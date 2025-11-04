// commands/correlation/helpers.rs
// Fonctions utilitaires pour parsing datetime et calcul volatilité
// Conforme .clinerules: < 150 lignes, pas d'unwrap()

use chrono::{NaiveDateTime, Duration, Timelike, DateTime, Utc, TimeZone};
use crate::services::CsvLoader;

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
    
    Err(format!("Cannot parse datetime from any known format: '{}'", s))
}

/// Calcule la volatilité moyenne dans une fenêtre de temps autour d'un événement
/// Returns volatility in pips (price difference * 10000 for most pairs)
pub fn calculate_volatility_from_csv(
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    window_minutes: i64,
) -> Result<f64, String> {
    let loader = CsvLoader::new();
    let candles = loader.load_candles(pair_symbol)
        .map_err(|e| format!("Failed to load candles for {}: {}", pair_symbol, e))?;
    
    if candles.is_empty() {
        return Ok(0.0);
    }
    
    // Convertir NaiveDateTime en DateTime<Utc> pour la comparaison
    let event_dt = Utc.from_utc_datetime(&event_datetime);
    let window_start = event_dt - Duration::minutes(window_minutes);
    let window_end = event_dt + Duration::minutes(window_minutes);
    
    let mut total_volatility = 0.0;
    let mut count = 0;
    
    for candle in candles.iter() {
        if candle.datetime >= window_start && candle.datetime <= window_end {
            let volatility = (candle.high - candle.low) * 10000.0; // Convert to pips
            total_volatility += volatility;
            count += 1;
        }
    }
    
    if count == 0 {
        Ok(0.0)
    } else {
        Ok(total_volatility / count as f64)
    }
}

/// Calcule la volatilité baseline (moyenne sur N jours à la même heure, excluant le jour de l'événement)
pub fn calculate_baseline_volatility_from_csv(
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    days_back: i64,
) -> Result<f64, String> {
    let loader = CsvLoader::new();
    let candles = loader.load_candles(pair_symbol)
        .map_err(|e| format!("Failed to load candles for {}: {}", pair_symbol, e))?;
    
    if candles.is_empty() {
        return Ok(0.0);
    }
    
    // Convertir NaiveDateTime en DateTime<Utc>
    let event_dt = Utc.from_utc_datetime(&event_datetime);
    let event_hour = event_dt.hour();
    let start_date = event_dt - Duration::days(days_back);
    let event_date = event_dt.date_naive();
    
    let mut total_volatility = 0.0;
    let mut count = 0;
    
    for candle in candles.iter() {
        let candle_date = candle.datetime.date_naive();
        // Dans la période, même heure, mais pas le jour de l'événement
        if candle.datetime >= start_date 
           && candle.datetime < event_dt 
           && candle_date != event_date
           && candle.datetime.hour() == event_hour {
            let volatility = (candle.high - candle.low) * 10000.0;
            total_volatility += volatility;
            count += 1;
        }
    }
    
    if count == 0 {
        Ok(0.0)
    } else {
        Ok(total_volatility / count as f64)
    }
}
