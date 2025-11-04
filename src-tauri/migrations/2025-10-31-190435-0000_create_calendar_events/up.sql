-- Migration: Créer table calendar_events

CREATE TABLE calendar_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    symbol TEXT NOT NULL,
    event_time TIMESTAMP NOT NULL,
    impact TEXT NOT NULL,
    description TEXT NOT NULL,
    actual REAL,
    forecast REAL,
    previous REAL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_calendar_events_symbol ON calendar_events(symbol);
CREATE INDEX idx_calendar_events_time ON calendar_events(event_time);
