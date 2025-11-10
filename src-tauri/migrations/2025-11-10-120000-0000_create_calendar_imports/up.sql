-- Migration: Créer table pour tracker les imports de calendriers
-- Permet de gérer plusieurs calendriers importés séparément

CREATE TABLE calendar_imports (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    event_count INTEGER NOT NULL DEFAULT 0,
    oldest_event_date TIMESTAMP,
    newest_event_date TIMESTAMP,
    imported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT 1
);

-- Index pour recherches rapides
CREATE INDEX idx_calendar_imports_is_active 
    ON calendar_imports(is_active);

-- Ajouter colonne calendar_import_id à calendar_events pour tracker
-- l'origine des événements
ALTER TABLE calendar_events 
ADD COLUMN calendar_import_id INTEGER;

-- Créer la contrainte de clé étrangère (soft FK pour compatibilité)
-- CREATE INDEX idx_calendar_events_import_id 
--     ON calendar_events(calendar_import_id);
