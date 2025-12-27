// models/calendar_event.rs - Événement économique
// Respect .clinerules: structs séparés Queryable vs Insertable

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::calendar_events;

/// Événement du calendrier économique (pour SELECT queries)
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = calendar_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CalendarEvent {
    pub id: i32,
    pub symbol: String,
    pub event_time: NaiveDateTime,
    pub impact: String,
    pub description: String,
    pub actual: Option<f64>,
    pub forecast: Option<f64>,
    pub previous: Option<f64>,
    pub created_at: NaiveDateTime,
    pub calendar_import_id: i32,
}

/// Structure pour insérer un nouvel événement (INSERT)
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = calendar_events)]
pub struct NewCalendarEvent {
    pub symbol: String,
    pub event_time: NaiveDateTime,
    pub impact: String,
    pub description: String,
    pub actual: Option<f64>,
    pub forecast: Option<f64>,
    pub previous: Option<f64>,
    pub calendar_import_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_calendar_event_creation() {
        let event = NewCalendarEvent {
            symbol: "EUR/USD".to_string(),
            event_time: Utc::now().naive_utc(),
            impact: "HIGH".to_string(),
            description: "Fed Rate Decision".to_string(),
            actual: Some(5.50),
            forecast: Some(5.25),
            previous: Some(5.00),
            calendar_import_id: 1,
        };

        assert_eq!(event.symbol, "EUR/USD");
        assert_eq!(event.impact, "HIGH");
        assert_eq!(event.actual, Some(5.50));
    }
}
