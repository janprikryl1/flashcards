use axum::extract::State;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use http::StatusCode;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sqlx::Row;
use crate::dto::app_state::AppState;
use time::Duration as TimeDuration;
use crate::utils::utils::extract_user_id;
use crate::dto::user::claims::Claims;
use crate::dto::user::login_payload::LoginPayload;
use crate::dto::user::me_response::MeResponse;
use crate::dto::user::register_payload::RegisterPayload;

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterPayload>) -> Result<StatusCode, (StatusCode, String)> {
    if payload.email.trim().is_empty() || payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Email nebo heslo je krátké".into()));
    }

    let email_exists = sqlx::query("SELECT 1 FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await
        .map_err(in500)?;
    if email_exists.is_some() {
        return Err((StatusCode::CONFLICT, "Email už existuje".into()));
    }

    let hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).map_err(in500)?;
    let now = Utc::now().to_rfc3339();

    let res = sqlx::query("INSERT INTO users (email, password_hash, created_at) VALUES (?, ?, ?)")
        .bind(&payload.email)
        .bind(&hash)
        .bind(now)
        .execute(&state.pool).await;

    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err(in500(e)),
    }
}

pub async fn login(State(state): State<AppState>, jar: CookieJar, Json(payload): Json<LoginPayload>) -> Result<(CookieJar, StatusCode), (StatusCode, String)> {
    let row = sqlx::query("SELECT id, password_hash FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&state.pool).await
        .map_err(in500)?;

    let Some(row) = row else {
        return Err((StatusCode::UNAUTHORIZED, "Neplatné přihlášení".into()));
    };

    let id: i64 = row.get("id");
    let hash: String = row.get("password_hash");

    if !bcrypt::verify(&payload.password, &hash).map_err(in500)? {
        return Err((StatusCode::UNAUTHORIZED, "Špatné heslo".into()));
    }

    let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
    let claims = Claims { sub: id, exp };
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).map_err(in500)?;

    let mut cookie = Cookie::new("session", token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_max_age(TimeDuration::days(7));

    Ok((jar.add(cookie), StatusCode::NO_CONTENT))
}

pub async fn me(State(state): State<AppState>, jar: CookieJar) -> Result<Json<MeResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(&jar, &state);
    if user_id.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
    let user_id = user_id.unwrap();

    let user = sqlx::query("SELECT id, email FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.pool).await
        .map_err(in500)?;

    Ok(Json(MeResponse {
        id: user.get("id"),
        email: user.get("email"),
    }))
}

pub async fn logout(_state: State<AppState>, jar: CookieJar) -> Result<(CookieJar, StatusCode), (StatusCode, String)> {
    let mut cookie = Cookie::new("session", "");
    cookie.set_path("/");
    cookie.set_max_age(TimeDuration::seconds(0));

    Ok((jar.remove(cookie), StatusCode::NO_CONTENT))
}

fn in500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Server error: {e}"))
}
