use crate::models::{Result, VolatilityError};
use chrono::{DateTime, NaiveDateTime, Utc};

/// Parse une date depuis un timestamp Unix ou format "YYYY.MM.DD HH:MM:SS"
pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    if let Ok(timestamp) = s.parse::<i64>() {
        return DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
            VolatilityError::ChronoError(format!("Invalid Unix timestamp: {}", timestamp))
        });
    }

    let normalized = s.replace('.', "-");
    NaiveDateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S")
        .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
        .map_err(|e| VolatilityError::ChronoError(format!("Cannot parse datetime '{}': {}", s, e)))
}

/// Parse un prix (f64) avec gestion d'erreur
pub fn parse_price(s: &str, field_name: &str, line: usize) -> Result<f64> {
    s.parse::<f64>().map_err(|e| {
        VolatilityError::InvalidCsvData(format!(
            "Line {}: Invalid {} value '{}': {}",
            line, field_name, s, e
        ))
    })
}

/// Reconstruit les nombres au format EU (0,583 → 0.583)
#[allow(dead_code)]
pub fn reconstruct_eu_number(int_part: &str, decimal_part: &str) -> String {
    format!("{}.{}", int_part, decimal_part)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_datetime() {
        let result = parse_datetime("1704117600");
        assert!(result.is_ok());
        let dt = result.expect("Failed to parse datetime");
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);

        let result = parse_datetime("2025.01.01 00:00:00");
        assert!(result.is_ok());
        let dt = result.expect("Failed to parse datetime");
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
}
