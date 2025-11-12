use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarFileInfo {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
    pub created: String,
    pub modified: String,
    pub event_count: Option<i64>,
    pub date_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairFileInfo {
    pub filename: String,
    pub path: String,
    pub pair: Option<String>,
    pub timeframe: Option<String>,
    pub period: Option<String>,
    pub size_bytes: u64,
    pub line_count: Option<usize>,
    pub date_range: Option<String>,
    pub created: String,
    pub modified: String,
}
