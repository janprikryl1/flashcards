use axum::{Json, extract::State};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use crate::utils::utils::extract_user_id;
use crate::dto::app_state::AppState;
use crate::dto::study_history::{StudyHistoryCreateDTO, StudyHistoryDTO};

pub(crate) async fn save_history(State(state): State<AppState>, jar: CookieJar, Json(payload): Json<StudyHistoryCreateDTO>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&jar, &state);
    if user_id.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
    let user_id = user_id.unwrap();

    let res = sqlx::query("INSERT INTO study_history (user_id, deck_id, accuracy) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(&payload.deck_id)
        .bind(payload.accuracy)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub(crate) async fn get_history(State(state): State<AppState>, jar: CookieJar) -> Result<Json<Vec<StudyHistoryDTO>>, StatusCode> {
    let user_id = extract_user_id(&jar, &state);
    if user_id.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let user_id = user_id.unwrap();

    let rows = sqlx::query_as::<_, StudyHistoryDTO>("
            SELECT sh.*, d.name AS deck_name
            FROM study_history sh
            JOIN decks d ON sh.deck_id = d.id
            WHERE user_id = ?
            ORDER BY sh.filled_at DESC
        ")
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows))
}

pub(crate) async fn delete_history_entry(State(state): State<AppState>, jar: CookieJar, Path(id): Path<i64>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&jar, &state);
    if user_id.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
    let user_id = user_id.unwrap();

    let result = sqlx::query("DELETE FROM study_history WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await;
    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                Err((StatusCode::NOT_FOUND, "Entry not found".into()))
            } else {
                Ok(StatusCode::NO_CONTENT)
            }
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}