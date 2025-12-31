use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::{env, str::FromStr};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL")?;

        let opts = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .disable_statement_logging()
            .to_owned();

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?;

        sqlx::query("PRAGMA journal_mode = WAL;").execute(&pool).await?;
        sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;

        // Tables
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS decks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                color TEXT NOT NULL
            );
        "#).execute(&pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                question TEXT NOT NULL,
                answer TEXT NOT NULL,
                deck_id INTEGER NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
            );
        "#).execute(&pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
        "#).execute(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
