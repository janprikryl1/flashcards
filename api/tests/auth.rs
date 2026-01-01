mod common;

#[cfg(test)]
mod auth_tests {
    use tower::ServiceExt;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::Router;
    use axum::routing::post;
    use http::StatusCode;
    use crate::common::{create_state, setup_test_db};
    use api::routes::authenticate;

    #[tokio::test]
    async fn test_user_registration_and_login() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/register", post(authenticate::register))
            .route("/api/login", post(authenticate::login))
            .with_state(state);

        //Registration
        let payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        //Login
        let login_payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/login")
            .header("content-type", "application/json")
            .body(Body::from(login_payload))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_user_registration_short_password() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/register", post(authenticate::register))
            .route("/api/login", post(authenticate::login))
            .with_state(state);

        //Registration
        let payload = r#"{"email": "test@example.com", "password": ""}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }


    #[tokio::test]
    async fn test_user_registration_and_login_invalid_password() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/register", post(authenticate::register))
            .route("/api/login", post(authenticate::login))
            .with_state(state);


        //Registration
        let payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        //Login
        let login_payload = r#"{"email": "test@example.com", "password": "password321"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/login")
            .header("content-type", "application/json")
            .body(Body::from(login_payload))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_user_registration_email_already_exists() {
        let pool = setup_test_db().await;
        let state = create_state(pool.clone());

        let app = Router::new()
            .route("/api/register", post(authenticate::register))
            .route("/api/login", post(authenticate::login))
            .with_state(state);


        //Registration 1
        let payload = r#"{"email": "test@example.com", "password": "password123"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        //Registration 2
        let payload = r#"{"email": "test@example.com", "password": "password321"}"#;
        let req = Request::builder()
            .method("POST")
            .uri("/api/register")
            .header("content-type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}