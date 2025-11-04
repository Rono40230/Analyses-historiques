-- Migration pour créer les tables du calendrier économique
CREATE TABLE calendar_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    symbol TEXT NOT NULL,
    event_time TIMESTAMP NOT NULL,
    event_type TEXT NOT NULL,
    impact_level TEXT NOT NULL,
    actual_value REAL,
    forecast_value REAL,
    previous_value REAL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_calendar_events_symbol ON calendar_events(symbol);
CREATE INDEX idx_calendar_events_time ON calendar_events(event_time);
CREATE INDEX idx_calendar_events_impact ON calendar_events(impact_level);

CREATE TABLE predicted_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    event_id INTEGER NOT NULL,
    predicted_probability REAL NOT NULL,
    confidence_score REAL NOT NULL,
    model_version TEXT NOT NULL,
    predicted_volatility_increase REAL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (event_id) REFERENCES calendar_events(id) ON DELETE CASCADE
);

CREATE INDEX idx_predicted_events_event ON predicted_events(event_id);
CREATE INDEX idx_predicted_events_confidence ON predicted_events(confidence_score);
