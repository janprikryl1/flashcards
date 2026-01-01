use axum::{Json, extract::State};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use crate::dto::app_state::AppState;
use crate::dto::cards::card::{FlashcardCreateDTO, FlashcardDTO, FlashcardPatchDTO};
use crate::dto::cards::cards_count::CardCount;

pub async fn card_count(State(state): State<AppState>) -> Result<Json<CardCount>, (StatusCode, String)> {
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

pub async fn list_cards(State(state): State<AppState>) -> Result<Json<Vec<FlashcardDTO>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, FlashcardDTO>("SELECT id, question, answer, deck_id, created_at FROM cards")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows))
}

pub async fn get_card(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<FlashcardDTO>, StatusCode> {
    let row = sqlx::query_as::<_, FlashcardDTO>("SELECT id, question, answer, deck_id, created_at FROM cards WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(row))
}

pub async fn create_card(State(state): State<AppState>, Json(payload): Json<FlashcardCreateDTO>) -> Result<Json<FlashcardDTO>, StatusCode> {
    let result = sqlx::query("INSERT INTO cards (question, answer, deck_id) VALUES (?, ?, ?)")
        .bind(&payload.question)
        .bind(&payload.answer)
        .bind(&payload.deck_id)
        .execute(&state.pool)
        .await;
    match result {
        Ok(res) => {
            let card = sqlx::query_as::<_, FlashcardDTO>("SELECT id, question, answer, deck_id, created_at FROM cards WHERE id = ?")
                .bind(res.last_insert_rowid())
                .fetch_one(&state.pool)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
            Ok(Json(card))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn update_card(State(state): State<AppState>, Path(id): Path<i64>, Json(payload): Json<FlashcardPatchDTO>) -> Result<Json<FlashcardDTO>, StatusCode> {
    let result = sqlx::query("UPDATE cards SET question = ?, answer = ?, deck_id = ? WHERE id = ?")
        .bind(&payload.question)
        .bind(&payload.answer)
        .bind(&payload.deck_id)
        .bind(id)
        .execute(&state.pool)
        .await;
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    if result.unwrap().rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query_as::<_, FlashcardDTO>("SELECT id, question, answer, deck_id, created_at FROM cards WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

pub async fn delete_card(State(state): State<AppState>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM cards WHERE id = ?")
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

pub async fn import_cards(State(state): State<AppState>, Json(payload): Json<Vec<FlashcardCreateDTO>>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut tx = state.pool.begin() //AI
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB Error".to_string()))?;

    for card in payload {
        sqlx::query("INSERT INTO cards (question, answer, deck_id) VALUES (?, ?, ?)")
            .bind(&card.question)
            .bind(&card.answer)
            .bind(&card.deck_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Insert failed".into()))?;
    }

    tx.commit()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Commit failed".to_string()))?;

    Ok(StatusCode::CREATED)
}