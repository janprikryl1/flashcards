use std::sync::Arc;
use axum::{routing::get, Router, Json, Extension};
use axum::extract::Path;

use crate::database::Database;
use crate::dto::post::Post;

pub async fn get_one(Path(id): Path<i64>, Extension(db): Extension<Arc<Database>>, ) -> Json<Post> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT title, content FROM posts WHERE id = ?",
    )
        .bind(id)
        .fetch_one(db.pool())
        .await
        .unwrap();

    Json(post)
}

pub async fn create(Extension(db): Extension<Arc<Database>>, Json(post): Json<Post>, ) {
    sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?);")
        .bind(post.title)
        .bind(post.content)
        .execute(db.pool())
        .await
        .unwrap();
}

pub async fn list(Extension(db): Extension<Arc<Database>>, ) -> Json<Vec<Post>> {
    let rows = sqlx::query_as::<_, Post>("SELECT title, content FROM posts")
        .fetch_all(db.pool())
        .await
        .unwrap();

    Json(rows)
}
