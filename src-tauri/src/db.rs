use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

pub fn init_db(app_data_dir: &PathBuf) -> Result<Connection> {
    std::fs::create_dir_all(app_data_dir)?;
    let db_path = app_data_dir.join("reader.db");
    let conn = Connection::open(db_path)?;
    apply_migrations(&conn)?;
    Ok(conn)
}

fn apply_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id TEXT PRIMARY KEY,
            exam_type TEXT NOT NULL,
            source TEXT,
            title TEXT NOT NULL,
            category TEXT,
            tags TEXT,
            difficulty TEXT,
            file_path TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS reading_progress (
            article_id TEXT PRIMARY KEY,
            scroll_position REAL NOT NULL DEFAULT 0,
            progress_percent REAL NOT NULL DEFAULT 0,
            last_read_at INTEGER NOT NULL,
            read_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS favorites (
            article_id TEXT PRIMARY KEY,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}

pub fn now_ts() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}