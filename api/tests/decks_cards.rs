mod common;

#[cfg(test)]
mod auth_tests {
    use tower::ServiceExt;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::Router;
    use axum::routing::{get, post};
    use http::StatusCode;
    use crate::common::{create_state, setup_test_db};
    use api::routes::{cards, decks};

    #[tokio::test]
    async fn create_deck_and_card() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/deck", post(decks::create_deck))
            .route("/api/card", post(cards::create_card))
            .route("/api/cards", get(cards::list_cards))
            .with_state(state);

        //Create deck
        let deck_payload = r##"{"name": "Test Deck", "description": "Desc", "color": "#ffffff"}"##;
        let req = Request::builder()
            .method("POST")
            .uri("/api/deck")
            .header("content-type", "application/json")
            .body(Body::from(deck_payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        //Create card
        let card_payload = r#"{"question": "Test question?", "answer": "Test answer", "deck_id": 1}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/card")
            .header("content-type", "application/json")
            .body(Body::from(card_payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        //Load cards
        let req = Request::builder()
            .method("GET")
            .uri("/api/cards")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}