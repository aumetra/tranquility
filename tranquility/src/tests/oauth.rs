use {
    super::{register::register_user, test_state},
    crate::api::oauth::token::AccessTokenResponse,
    std::sync::Arc,
    warp::{hyper::body, Reply},
};

#[tokio::test]
/// Test that requesting an OAuth access token by using the password grant (authenticate via username/password directly)
async fn password_grant() {
    let state = Arc::new(test_state().await);

    let register_response = register_user(&state, "oauth_password_grant", "1234567.").await;
    assert!(register_response.status().is_success());

    let oauth_request = "grant_type=password&username=oauth_password_grant&password=1234567.";
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
    let response: AccessTokenResponse = serde_json::from_slice(&response_data).unwrap();

    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.scope, "read write follow push");
}
