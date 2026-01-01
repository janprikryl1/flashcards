mod common;

#[cfg(test)]
mod auth_tests {
    use tower::ServiceExt;
    use axum::body::{to_bytes, Body};
    use axum::extract::Request;
    use axum::Router;
    use axum::routing::{post};
    use http::StatusCode;
    use serde_json::Value;
    use crate::common::{create_state, setup_test_db};
    use api::routes::{authenticate, decks, study_history};

    #[tokio::test]
    async fn save_history() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/register", post(authenticate::register))
            .route("/api/login", post(authenticate::login))
            .route("/api/deck", post(decks::create_deck))
            .route("/api/study-history", post(study_history::save_history))
            .with_state(state);

        let payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let login_payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/login")
            .header("content-type", "application/json")
            .body(Body::from(login_payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let cookie_header = response.headers().get("set-cookie")
            .expect("Missing cookie")
            .to_str()
            .unwrap()
            .to_string();

        let deck_payload = r##"{"name": "Math exam", "description": "Test", "color": "#ff0000"}"##;

        let req = Request::builder()
            .method("POST")
            .uri("/api/deck")
            .header("content-type", "application/json")
            .header("Cookie", &cookie_header)
            .body(Body::from(deck_payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        //AI
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let deck_json: Value = serde_json::from_slice(&body_bytes).unwrap();
        let deck_id = deck_json.get("id").expect("Response missing id").as_i64().unwrap();

        let history_payload = format!(r#"{{"deck_id": {}, "accuracy": 0.95}}"#, deck_id);
        let req = Request::builder()
            .method("POST")
            .uri("/api/study-history")
            .header("content-type", "application/json")
            .header("Cookie", &cookie_header)
            .body(Body::from(history_payload))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED, "Failed to save study history");
    }
}