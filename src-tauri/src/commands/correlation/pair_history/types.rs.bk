use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PairEventHistoryItem {
    pub event_id: i32,
    pub datetime: String,
    pub event_name: String,
    pub impact: String,
    pub volatility: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatility_formatted: Option<String>,
    pub change_percent: f64,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopEvent {
    pub name: String,
    pub datetime: String,
    pub volatility: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairEventHistory {
    pub symbol: String,
    pub period: String,
    pub total_events: i32,
    pub avg_volatility: f64,
    pub max_volatility: f64,
    pub avg_multiplier: f64,
    pub events: Vec<PairEventHistoryItem>,
    pub top_events: Vec<TopEvent>,
}
