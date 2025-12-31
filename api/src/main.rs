mod database;
mod dto;
mod routes;

use std::sync::Arc;
use axum::{routing::{get, post}, Router, Extension};
use axum::http::Method;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};
use crate::routes::{cards, decks};
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
        .route("/", get(|| async { "Backend for flashcards!" }))

        .route("/api/card-count", get(cards::card_count))

        .route("/api/deck", post(decks::create_deck))
        .route("/api/deck/:id", get(decks::get_deck).put(decks::update_deck).delete(decks::delete_deck))
        .route("/api/decks", get(decks::list_decks))

        .route("/api/card", post(cards::create_card))
        .route("/api/card/:id", get(cards::get_card).put(cards::update_card).delete(cards::delete_card))
        .route("/api/cards", get(cards::list_cards))

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
