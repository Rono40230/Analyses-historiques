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
