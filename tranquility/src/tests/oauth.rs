use {
    super::{register::register_user, test_state, TEST_PASSWORD},
    crate::{api::oauth::token::AccessTokenResponse, state::ArcState},
    std::sync::Arc,
    warp::{hyper::body, Reply},
};

pub async fn obtain_access_token(state: &ArcState, username: &str) -> AccessTokenResponse {
    let oauth_request = format!(
        "grant_type=password&username={}&password={}",
        username, TEST_PASSWORD
    );
    let oauth_routes = crate::api::oauth::routes(&state);

    let response = warp::test::request()
        .method("POST")
        .path("/oauth/token")
        .body(&oauth_request)
        .filter(&oauth_routes)
        .await
        .expect("Unsuccessful request")
        .into_response();

    assert!(response.status().is_success());

    let response_data = body::to_bytes(response.into_body()).await.unwrap();

    serde_json::from_slice(&response_data).unwrap()
}

#[tokio::test]
/// Test that requesting an OAuth access token by using the password grant (authenticate via username/password directly)
async fn password_grant() {
    let state = Arc::new(test_state().await);

    let username = "oauth_password_grant";
    register_user(&state, username).await;

    let response = obtain_access_token(&state, username).await;

    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.scope, "read write follow push");
}
