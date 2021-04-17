use {
    super::test_state,
    crate::state::ArcState,
    std::sync::Arc,
    warp::{
        hyper::{body, StatusCode},
        reply::Response,
        Reply,
    },
};

pub async fn register_user(state: &ArcState, username: &str, password: &str) -> Response {
    let body = format!(
        "username={username}&email={username}@example.com&password={password}",
        username = username,
        password = password,
    );

    let register_endpoint = crate::api::register::routes(state);
    warp::test::request()
        .method("POST")
        .path("/api/tranquility/v1/register")
        .body(body)
        .filter(&register_endpoint)
        .await
        .expect("Unsuccessful request")
        .into_response()
}

#[tokio::test]
async fn closed_registrations() {
    let mut state = test_state().await;
    state.config.instance.closed_registrations = true;
    let state = Arc::new(state);

    let test_response = register_user(&state, "test_closed_register", "1234.").await;
    assert_eq!(test_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn register_endpoint() {
    let state = Arc::new(test_state().await);

    let test_response = register_user(&state, "test_register", "1234.").await;
    assert_eq!(test_response.status(), StatusCode::CREATED);

    let body_data = body::to_bytes(test_response.into_body()).await.unwrap();
    assert_eq!(body_data, b"Account created" as &'static [u8]);
}
