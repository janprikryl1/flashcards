use std::sync::Arc;
use axum::{Json, extract::State, Extension};
use axum::extract::Path;
use http::StatusCode;
use api::database::Database;
use crate::dto::app_state::AppState;
use crate::dto::cards::cards_count::CardCount;
use crate::dto::decks::deck::{DeckCreateDTO, DeckDTO};
use crate::dto::post::Post;
use crate::dto::register_payload::RegisterPayload;

pub(crate) async fn create_deck(State(state): State<AppState>, Json(payload): Json<DeckCreateDTO>) -> Result<Json<DeckDTO>, (StatusCode, String)> {
    let res = sqlx::query("INSERT INTO decks (name, description, color) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.color)
        .execute(&state.pool).await;

    match res {
        Ok(result) => {
            let deck_id = result.last_insert_rowid();
            let deck = DeckDTO {
                id: deck_id,
                name: payload.name,
                description: payload.description,
                color: payload.color,
            };
            Ok(Json(deck))
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub(crate) async fn list_decks(State(state): State<AppState>) -> Result<Json<Vec<DeckDTO>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, DeckDTO>("SELECT id, name, description, color FROM decks")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows))
}

pub(crate) async fn get_deck(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<DeckDTO>, StatusCode> {
    let row = sqlx::query_as::<_, DeckDTO>("SELECT id, name, description, color FROM decks WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(row))
}

pub(crate) async fn update_deck(State(state): State<AppState>, Path(id): Path<i64>, Json(payload): Json<DeckCreateDTO>) -> Result<Json<DeckDTO>, StatusCode> {
    let result = sqlx::query("UPDATE decks SET name = ?, description = ?, color = ? WHERE id = ?")
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.color)
        .bind(id)
        .execute(&state.pool)
        .await;
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    if result.unwrap().rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let updated_deck = DeckDTO {
        id,
        name: payload.name,
        description: payload.description,
        color: payload.color,
    };

    Ok(Json(updated_deck))
}

pub(crate) async fn delete_deck(State(state): State<AppState>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    //Delete all cards in this deck
    let result = sqlx::query("DELETE FROM cards WHERE deck_id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    //Delete the deck itself
    let result = sqlx::query("DELETE FROM decks WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    if result.unwrap().rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}