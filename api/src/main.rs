mod database;
mod models;
mod routes;

use std::sync::Arc;
use axum::{routing::get, Router, Extension};
use crate::routes::posts::{create, get_one, list};

#[tokio::main]
async fn main() {
    let db = Arc::new(database::Database::new().await.expect("init db"));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/posts", get(list).post(create))
        .route("/api/posts/:id", get(get_one))
        .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
