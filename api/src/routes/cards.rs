use axum::{Json, extract::State};
use http::StatusCode;
use crate::dto::app_state::AppState;
use crate::dto::cards::cards_count::CardCount;

pub(crate) async fn card_count(State(state): State<AppState>) -> Result<Json<CardCount>, (StatusCode, String)> {
    let pool = &state.pool;

    let cards_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cards")
        .fetch_one(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let decks_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM decks")
        .fetch_one(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CardCount {
        cards: cards_count,
        decks: decks_count,
    }))
}