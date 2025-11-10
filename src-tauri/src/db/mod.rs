// db/mod.rs - Configuration Diesel SQLite

pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

pub fn create_pool(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)?;
    Ok(Arc::new(pool))
}

/// Crée la table calendar_events si elle n'existe pas
pub fn ensure_calendar_table(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;
    
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            symbol TEXT NOT NULL,
            event_time TIMESTAMP NOT NULL,
            impact TEXT NOT NULL,
            description TEXT NOT NULL,
            actual REAL,
            forecast REAL,
            previous REAL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_calendar_events_symbol ON calendar_events(symbol)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_calendar_events_time ON calendar_events(event_time)"
    ).execute(&mut conn)?;
    
    Ok(())
}

/// Crée les tables de la DB paires (candle_data, pair_metadata, import_log)
/// Utilisée lors de l'initialisation du pool pairs.db
#[allow(dead_code)]
pub fn ensure_pair_tables(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;
    
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS candle_data (
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
        )"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_candle_data_symbol_timeframe_time 
            ON candle_data(symbol, timeframe, time)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_candle_data_time ON candle_data(time)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_candle_data_source_file ON candle_data(source_file)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS pair_metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            symbol TEXT NOT NULL,
            timeframe TEXT NOT NULL,
            row_count INTEGER NOT NULL DEFAULT 0,
            last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_imported_file TEXT,
            data_quality_score REAL DEFAULT 1.0,
            UNIQUE(symbol, timeframe)
        )"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_pair_metadata_symbol ON pair_metadata(symbol)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS import_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            filename TEXT NOT NULL,
            symbol TEXT NOT NULL,
            timeframe TEXT NOT NULL,
            imported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            row_count INTEGER NOT NULL,
            expected_row_count INTEGER,
            status TEXT NOT NULL,
            error_message TEXT,
            checksum TEXT
        )"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_import_log_imported_at ON import_log(imported_at)"
    ).execute(&mut conn)?;
    
    diesel::sql_query(
        "CREATE INDEX IF NOT EXISTS idx_import_log_symbol ON import_log(symbol, timeframe)"
    ).execute(&mut conn)?;
    
    Ok(())
}
