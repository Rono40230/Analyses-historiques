-- Migration: Créer tables pour la DB paires (pairs.db)
-- Cette DB stocke les données de trading (candles) importées des fichiers CSV
-- Séparation intentionnelle: calendrier.db (économique) + pairs.db (trading)

CREATE TABLE candle_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    time TIMESTAMP NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume REAL NOT NULL,
    imported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source_file TEXT NOT NULL
);

-- Index principal: recherches par paire + timeframe + temps
CREATE UNIQUE INDEX idx_candle_data_symbol_timeframe_time 
    ON candle_data(symbol, timeframe, time);

-- Index secondaire: recherches par temps (pour séries temporelles)
CREATE INDEX idx_candle_data_time 
    ON candle_data(time);

-- Index pour maintenance: trouver candles importées depuis un fichier
CREATE INDEX idx_candle_data_source_file 
    ON candle_data(source_file);


-- Table métadonnées: dernier import par paire/timeframe
CREATE TABLE pair_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    row_count INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_imported_file TEXT,
    data_quality_score REAL DEFAULT 1.0,
    UNIQUE(symbol, timeframe)
);

CREATE INDEX idx_pair_metadata_symbol 
    ON pair_metadata(symbol);


-- Table audit: trace d'imports pour validation intégrité
CREATE TABLE import_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    filename TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    imported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    row_count INTEGER NOT NULL,
    expected_row_count INTEGER,
    status TEXT NOT NULL,  -- 'success', 'warning', 'failed'
    error_message TEXT,
    checksum TEXT
);

CREATE INDEX idx_import_log_imported_at 
    ON import_log(imported_at);

CREATE INDEX idx_import_log_symbol 
    ON import_log(symbol, timeframe);
