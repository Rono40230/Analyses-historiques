// services/pair_data/formats.rs - Parseurs pour formats CSV spécifiques
// Conforme .clinerules : < 200L, pas d'unwrap()

use super::types::{CsvFormat, NormalizedCandle};
use super::datetime_parser::DateTimeParser;

/// Parseurs pour différents formats CSV
pub(super) struct FormatParsers;

impl FormatParsers {
    /// Parse un nombre avec virgule (EU) ou point (US) comme séparateur décimal
    fn parse_decimal(s: &str) -> Result<f64, String> {
        s.replace(',', ".").parse::<f64>().map_err(|_| format!("Nombre invalide: {}", s))
    }

    /// Parse une ligne selon le format détecté
    pub(super) fn parse_record(
        record: &csv::StringRecord,
        format: &CsvFormat,
        headers: &[String],
    ) -> Result<NormalizedCandle, String> {
        match format {
            CsvFormat::MetaTrader => Self::parse_metatrader(record),
            CsvFormat::TradingView => Self::parse_tradingview(record),
            CsvFormat::Dukascopy => Self::parse_dukascopy(record),
            CsvFormat::Generic => Self::parse_generic(record, headers),
        }
    }

    /// Parse format MetaTrader: Date,Time,Open,High,Low,Close,Volume
    fn parse_metatrader(record: &csv::StringRecord) -> Result<NormalizedCandle, String> {
        if record.len() < 7 {
            return Err("Ligne incomplète".to_string());
        }
        
        let date = record.get(0).ok_or("Date manquante")?;
        let time = record.get(1).ok_or("Time manquante")?;
        let datetime_str = format!("{} {}", date, time);
        
        // Formats possibles: "2024.01.01 12:00" ou "2024-01-01 12:00:00"
        let timestamp = DateTimeParser::parse(&datetime_str)?;
        
        Ok(NormalizedCandle {
            timestamp,
            open: Self::parse_decimal(record.get(2).ok_or("Open column missing")?)?,
            high: Self::parse_decimal(record.get(3).ok_or("High column missing")?)?,
            low: Self::parse_decimal(record.get(4).ok_or("Low column missing")?)?,
            close: Self::parse_decimal(record.get(5).ok_or("Close column missing")?)?,
            volume: Self::parse_decimal(record.get(6).ok_or("Volume column missing")?).unwrap_or(0.0),
        })
    }
    
    /// Parse format TradingView: time,open,high,low,close,volume
    fn parse_tradingview(record: &csv::StringRecord) -> Result<NormalizedCandle, String> {
        if record.len() < 6 {
            return Err("Ligne incomplète".to_string());
        }
        
        let time_str = record.get(0).ok_or("Time manquante")?;
        
        // TradingView peut utiliser Unix timestamp ou datetime
        let timestamp = if time_str.contains("-") || time_str.contains("/") {
            DateTimeParser::parse(time_str)?
        } else {
            time_str.parse::<i64>().map_err(|_| "Timestamp invalide")?
        };
        
        Ok(NormalizedCandle {
            timestamp,
            open: Self::parse_decimal(record.get(1).ok_or("Open column missing")?)?,
            high: Self::parse_decimal(record.get(2).ok_or("High column missing")?)?,
            low: Self::parse_decimal(record.get(3).ok_or("Low column missing")?)?,
            close: Self::parse_decimal(record.get(4).ok_or("Close column missing")?)?,
            volume: Self::parse_decimal(record.get(5).ok_or("Volume column missing")?).unwrap_or(0.0),
        })
    }
    
    /// Parse format Dukascopy: Gmt time,Open,High,Low,Close,Volume
    fn parse_dukascopy(record: &csv::StringRecord) -> Result<NormalizedCandle, String> {
        if record.len() < 6 {
            return Err("Ligne incomplète".to_string());
        }
        
        let time_str = record.get(0).ok_or("Gmt time manquante")?;
        let timestamp = DateTimeParser::parse(time_str)?;
        
        Ok(NormalizedCandle {
            timestamp,
            open: Self::parse_decimal(record.get(1).ok_or("Open column missing")?)?,
            high: Self::parse_decimal(record.get(2).ok_or("High column missing")?)?,
            low: Self::parse_decimal(record.get(3).ok_or("Low column missing")?)?,
            close: Self::parse_decimal(record.get(4).ok_or("Close column missing")?)?,
            volume: Self::parse_decimal(record.get(5).ok_or("Volume column missing")?).unwrap_or(0.0),
        })
    }
    
    /// Parse format générique
    fn parse_generic(record: &csv::StringRecord, headers: &[String]) -> Result<NormalizedCandle, String> {
        let headers_lower: Vec<String> = headers.iter().map(|h| h.to_lowercase()).collect();
        
        // Trouver les indices des colonnes
        let time_idx = headers_lower.iter().position(|h| h.contains("time") || h.contains("date"))
            .ok_or("Colonne timestamp non trouvée")?;
        let open_idx = headers_lower.iter().position(|h| h.contains("open"))
            .ok_or("Colonne open non trouvée")?;
        let high_idx = headers_lower.iter().position(|h| h.contains("high"))
            .ok_or("Colonne high non trouvée")?;
        let low_idx = headers_lower.iter().position(|h| h.contains("low"))
            .ok_or("Colonne low non trouvée")?;
        let close_idx = headers_lower.iter().position(|h| h.contains("close"))
            .ok_or("Colonne close non trouvée")?;
        let volume_idx = headers_lower.iter().position(|h| h.contains("volume"));
        
        let time_str = record.get(time_idx).ok_or("Timestamp manquant")?;
        let timestamp = DateTimeParser::parse(time_str)?;
        
        Ok(NormalizedCandle {
            timestamp,
            open: Self::parse_decimal(record.get(open_idx).ok_or("Open value missing")?)?,
            high: Self::parse_decimal(record.get(high_idx).ok_or("High value missing")?)?,
            low: Self::parse_decimal(record.get(low_idx).ok_or("Low value missing")?)?,
            close: Self::parse_decimal(record.get(close_idx).ok_or("Close value missing")?)?,
            volume: volume_idx.and_then(|idx| record.get(idx).and_then(|v| Self::parse_decimal(v).ok())).unwrap_or(0.0),
        })
    }
}
