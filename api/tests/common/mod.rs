use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use api::dto::app_state::AppState;
//AI

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create test pool");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, email TEXT NOT NULL UNIQUE, password_hash TEXT NOT NULL, created_at TEXT NOT NULL);")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS decks (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT NOT NULL, color TEXT NOT NULL);")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS cards (id INTEGER PRIMARY KEY AUTOINCREMENT, question TEXT NOT NULL, answer TEXT NOT NULL, deck_id INTEGER NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE);")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS study_history (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER NOT NULL, deck_id INTEGER NOT NULL, filled_at TEXT DEFAULT CURRENT_TIMESTAMP, accuracy FLOAT NOT NULL, FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE, FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE);")
        .execute(&pool)
        .await
        .unwrap();

    pool
}

pub fn create_state(pool: SqlitePool) -> AppState {
    AppState {
        pool,
        jwt_secret: "test-secret".to_string(),
    }
}