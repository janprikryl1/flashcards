use sqlx::{SqlitePool};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: SqlitePool,
    pub(crate) jwt_secret: String,
}