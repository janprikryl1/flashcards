mod database;
mod dto;
mod routes;

use std::sync::Arc;
use axum::{routing::{get, post}, Router, Extension};
use axum::http::Method;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};
use crate::routes::posts::{create, get_one, list};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use tokio::net::TcpListener;
use crate::dto::app_state::AppState;
use crate::routes::authenticate;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".into());

    let db = Arc::new(database::Database::new().await.expect("init db"));

    let state = AppState {
        pool: db.pool().clone(),
        jwt_secret: jwt_secret.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/posts", get(list).post(create))
        .route("/api/posts/:id", get(get_one))
        .route("/api/register", post(authenticate::register))
        .route("/api/login", post(authenticate::login))
        .route("/api/me", get(authenticate::me))
        .route("/api/logout", post(authenticate::logout))
        .with_state(state)
        .layer(cors)
        .layer(Extension(db));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
