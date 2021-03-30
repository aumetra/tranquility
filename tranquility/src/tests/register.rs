use {
    super::test_state,
    std::sync::Arc,
    warp::{
        hyper::{body, StatusCode},
        Reply,
    },
};

possibly_failing_test! {
    name => register_endpoint,
    body => {
        let state = Arc::new(test_state().await);
        let register_endpoint = crate::api::register::routes(&state);

        let test_request = warp::test::request()
            .method("POST")
            .path("/api/tranquility/v1/register")
            .body("username=test&email=test@example.com&password=test1234.")
            .filter(&register_endpoint);

        let test_response = test_request
            .await
            .expect("Unsuccessful request")
            .into_response();

        assert_eq!(test_response.status(), StatusCode::CREATED);

        let body_data = body::to_bytes(test_response.into_body()).await.unwrap();
        assert_eq!(body_data, b"Account created" as &'static [u8]);
    }
}

#[tokio::test]
async fn closed_registrations() {
    let mut state = test_state().await;
    state.config.instance.closed_registrations = true;
    let state = Arc::new(state);

    let register_endpoint = crate::api::register::routes(&state);

    let test_request = warp::test::request()
        .method("POST")
        .path("/api/tranquility/v1/register")
        .body("username=test&email=test@example.com&password=1234.")
        .filter(&register_endpoint);
    let test_response = test_request
        .await
        .expect("Unsuccessful request")
        .into_response();

    assert_eq!(test_response.status(), StatusCode::FORBIDDEN);
}
