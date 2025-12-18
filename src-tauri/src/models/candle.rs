// models/candle.rs - Structure représentant une bougie OHLCV
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Candle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub symbol: String,
    pub datetime: DateTime<Utc>,
    #[validate(range(min = 0.0))]
    pub open: f64,
    #[validate(range(min = 0.0))]
    pub high: f64,
    #[validate(range(min = 0.0))]
    pub low: f64,
    #[validate(range(min = 0.0))]
    pub close: f64,
    #[validate(range(min = 0.0))]
    pub volume: f64,
}

impl Candle {
    pub fn new(
        symbol: String,
        datetime: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> crate::models::Result<Self> {
        if high < low {
            return Err(crate::models::VolatilityError::ValidationError(
                "High cannot be lower than low".to_string(),
            ));
        }
        if high < open || high < close {
            return Err(crate::models::VolatilityError::ValidationError(
                "High must be greater than or equal to open and close".to_string(),
            ));
        }
        if low > open || low > close {
            return Err(crate::models::VolatilityError::ValidationError(
                "Low must be less than or equal to open and close".to_string(),
            ));
        }

        let candle = Self {
            id: None,
            symbol,
            datetime,
            open,
            high,
            low,
            close,
            volume,
        };
        
        candle.validate().map_err(|e| {
            crate::models::VolatilityError::ValidationError(e.to_string())
        })?;
        
        Ok(candle)
    }

    pub fn true_range(&self, prev_close: Option<f64>) -> f64 {
        let high_low = self.high - self.low;
        match prev_close {
            Some(pc) => {
                let high_close = (self.high - pc).abs();
                let low_close = (self.low - pc).abs();
                high_low.max(high_close).max(low_close)
            }
            None => high_low,
        }
    }

    pub fn body_range(&self) -> f64 {
        let range = self.high - self.low;
        if range == 0.0 {
            return 0.0;
        }
        (((self.close - self.open).abs()) / range) * 100.0
    }

    pub fn shadow_ratio(&self) -> f64 {
        let upper_wick = self.high - self.open.max(self.close);
        let lower_wick = self.open.min(self.close) - self.low;
        if lower_wick == 0.0 {
            return 1.0;
        }
        upper_wick / lower_wick
    }

    pub fn hour_utc(&self) -> u32 {
        self.datetime.hour()
    }
}

#[cfg(test)]
#[path = "candle_tests.rs"]
mod tests;
