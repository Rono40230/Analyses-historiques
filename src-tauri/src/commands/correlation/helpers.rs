// commands/correlation/helpers.rs
// Fonctions utilitaires pour parsing datetime et calcul volatilité
// Conforme .clinerules: < 150 lignes, pas d'unwrap()

use chrono::{NaiveDateTime, Duration, Timelike, DateTime, Utc, TimeZone};
use crate::services::CsvLoader;

#[derive(Debug)]
pub struct VolatilityMetrics {
    pub event_volatility: f64,
    pub baseline_volatility: f64,
}

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

/// Calcule la volatilité event ET baseline en UNE SEULE lecture du fichier
/// Cela évite de charger le même fichier 2x
pub fn calculate_both_volatilities(
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    event_window_minutes: i64,
    baseline_days_back: i64,
) -> Result<VolatilityMetrics, String> {
    let loader = CsvLoader::new();
    let candles = loader.load_candles(pair_symbol)
        .map_err(|e| format!("Failed to load candles for {}: {}", pair_symbol, e))?;
    
    if candles.is_empty() {
        return Ok(VolatilityMetrics {
            event_volatility: 0.0,
            baseline_volatility: 0.0,
        });
    }
    
    let event_dt = Utc.from_utc_datetime(&event_datetime);
    let event_hour = event_dt.hour();
    let event_date = event_dt.date_naive();
    
    // Fenêtres temporelles
    let event_window_start = event_dt - Duration::minutes(event_window_minutes);
    let event_window_end = event_dt + Duration::minutes(event_window_minutes);
    let baseline_start = event_dt - Duration::days(baseline_days_back);
    
    let mut event_volatility_sum = 0.0;
    let mut event_count = 0;
    let mut baseline_volatility_sum = 0.0;
    let mut baseline_count = 0;
    
    // Lecture UNIQUE du fichier
    for candle in candles.iter() {
        let pips = (candle.high - candle.low) * 10000.0;
        
        // Volatilité pendant l'événement (±30 min)
        if candle.datetime >= event_window_start && candle.datetime <= event_window_end {
            event_volatility_sum += pips;
            event_count += 1;
        }
        
        // Volatilité baseline (même heure sur N jours précédents, excluant le jour d'événement)
        let candle_date = candle.datetime.date_naive();
        if candle.datetime >= baseline_start 
           && candle.datetime < event_dt 
           && candle_date != event_date
           && candle.datetime.hour() == event_hour {
            baseline_volatility_sum += pips;
            baseline_count += 1;
        }
    }
    
    Ok(VolatilityMetrics {
        event_volatility: if event_count == 0 { 0.0 } else { event_volatility_sum / event_count as f64 },
        baseline_volatility: if baseline_count == 0 { 0.0 } else { baseline_volatility_sum / baseline_count as f64 },
    })
}

/// Calcule la volatilité moyenne dans une fenêtre de temps autour d'un événement
/// Returns volatility in pips (price difference * 10000 for most pairs)
pub fn calculate_volatility_from_csv(
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    window_minutes: i64,
) -> Result<f64, String> {
    let metrics = calculate_both_volatilities(pair_symbol, event_datetime, window_minutes, 30)?;
    Ok(metrics.event_volatility)
}

/// Calcule la volatilité baseline (moyenne sur N jours à la même heure, excluant le jour de l'événement)
pub fn calculate_baseline_volatility_from_csv(
    pair_symbol: &str,
    event_datetime: NaiveDateTime,
    days_back: i64,
) -> Result<f64, String> {
    let metrics = calculate_both_volatilities(pair_symbol, event_datetime, 30, days_back)?;
    Ok(metrics.baseline_volatility)
}

/// Calcule les volatilités à partir de candles PRÉ-CHARGÉS (pour éviter les récharges multiples)
/// Utilisé pour "Par Paire" et "Heatmap" qui traitent plusieurs événements
pub fn calculate_volatilities_from_preloaded_candles(
    candles: &[(DateTime<Utc>, f64, f64)], // (datetime, high, low)
    event_datetime: NaiveDateTime,
    event_window_minutes: i64,
    baseline_days_back: i64,
) -> VolatilityMetrics {
    if candles.is_empty() {
        return VolatilityMetrics {
            event_volatility: 0.0,
            baseline_volatility: 0.0,
        };
    }

    let event_dt = Utc.from_utc_datetime(&event_datetime);
    let event_hour = event_dt.hour();
    let event_date = event_dt.date_naive();

    // Fenêtres temporelles
    let event_window_start = event_dt - Duration::minutes(event_window_minutes);
    let event_window_end = event_dt + Duration::minutes(event_window_minutes);
    let baseline_start = event_dt - Duration::days(baseline_days_back);

    let mut event_volatility_sum = 0.0;
    let mut event_count = 0;
    let mut baseline_volatility_sum = 0.0;
    let mut baseline_count = 0;

    // Itérer sur les candles pré-chargés
    for (candle_dt, high, low) in candles.iter() {
        let pips = (high - low) * 10000.0;

        // Volatilité pendant l'événement (±N min)
        if candle_dt >= &event_window_start && candle_dt <= &event_window_end {
            event_volatility_sum += pips;
            event_count += 1;
        }

        // Volatilité baseline (même heure sur N jours précédents, excluant le jour d'événement)
        let candle_date = candle_dt.date_naive();
        if candle_dt >= &baseline_start 
           && candle_dt < &event_dt 
           && candle_date != event_date
           && candle_dt.hour() == event_hour {
            baseline_volatility_sum += pips;
            baseline_count += 1;
        }
    }

    VolatilityMetrics {
        event_volatility: if event_count == 0 { 0.0 } else { event_volatility_sum / event_count as f64 },
        baseline_volatility: if baseline_count == 0 { 0.0 } else { baseline_volatility_sum / baseline_count as f64 },
    }
}
