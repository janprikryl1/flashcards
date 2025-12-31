use axum_extra::extract::CookieJar;
use http::StatusCode;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::dto::app_state::AppState;
use crate::dto::user::claims::Claims;

pub fn extract_user_id(jar: &CookieJar, state: &AppState) -> Result<i64, (StatusCode, String)> {
    let token = jar
        .get("session")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing session".into()))?
        .value();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

    Ok(data.claims.sub)
}