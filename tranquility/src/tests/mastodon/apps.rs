use {
    crate::{
        api::mastodon::apps::RegisterForm, consts::oauth::DISPLAY_REDIRECT_URI, state::ArcState,
        tests::test_state,
    },
    std::sync::Arc,
    tranquility_types::mastodon::App,
    warp::{hyper::body, Reply},
};

/// Register a new OAuth application with the Mastodon specific app endpoint (panics on failure)
async fn create_app(state: &ArcState) -> App {
    let mastodon_api = crate::api::mastodon::routes(state);

    let register_form = RegisterForm {
        client_name: "test client".into(),
        redirect_uris: DISPLAY_REDIRECT_URI.into(),
        scopes: "read write follow push".into(),
        website: String::new(),
    };
    let response = warp::test::request()
        .path("/api/v1/apps")
        .json(&register_form)
        .filter(&mastodon_api)
        .await
        .expect("Unsuccessful request")
        .into_response();

    assert!(response.status().is_success());

    let response_data = body::to_bytes(response.into_body()).await.unwrap();

    serde_json::from_slice(&response_data).unwrap()
}

#[tokio::test]
/// Test that creating an app works
async fn create() {
    let state = Arc::new(test_state().await);

    let app = create_app(&state).await;
    assert_eq!(app.name, "test client");
    assert_eq!(app.redirect_uri, DISPLAY_REDIRECT_URI);
    assert_eq!(app.website, None);
}
