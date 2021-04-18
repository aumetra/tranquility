use {
    crate::{
        api::oauth::token::AccessTokenResponse,
        state::ArcState,
        tests::{oauth::obtain_access_token, register::register_user, test_state},
    },
    std::sync::Arc,
    tranquility_types::mastodon::Account,
    warp::{hyper::body, Reply},
};

/// Get the Mastodon account information of the user associated with the access token via the `verify_credentials` API (panics on failure)
async fn user_by_access_token(state: &ArcState, token: &AccessTokenResponse) -> Account {
    let mastodon_api = crate::api::mastodon::routes(&state);
    let response = warp::test::request()
        .path("/api/v1/accounts/verify_credentials")
        .header(
            "Authorization",
            format!("{} {}", token.token_type, token.access_token),
        )
        .filter(&mastodon_api)
        .await
        .expect("Unsuccessful request")
        .into_response();

    assert!(response.status().is_success());

    let data = body::to_bytes(response.into_body()).await.unwrap();
    serde_json::from_slice(&data).unwrap()
}

#[tokio::test]
/// Test that the `accounts/:id` endpoint returns valid data
async fn account_id() {
    let state = Arc::new(test_state().await);

    let username = "mastodon_account_id";
    register_user(&state, username).await;

    let access_token = obtain_access_token(&state, username).await;
    let mut account = user_by_access_token(&state, &access_token).await;

    let mastodon_api = crate::api::mastodon::routes(&state);

    let request_path = format!("/api/v1/accounts/{}", account.id);
    let response = warp::test::request()
        .path(&request_path)
        .filter(&mastodon_api)
        .await
        .expect("Unsuccessful request")
        .into_response();

    assert!(response.status().is_success());

    let data = body::to_bytes(response.into_body()).await.unwrap();
    let acct_by_id: Account = serde_json::from_slice(&data).unwrap();

    // Remove the source field
    account.source = None;

    assert_eq!(acct_by_id, account);
}

#[tokio::test]
/// Test that the `verify_credentials` endpoint works
async fn verify_credentials() {
    let state = Arc::new(test_state().await);

    let username = "mastodon_verify_credentails";
    register_user(&state, username).await;

    let access_token = obtain_access_token(&state, username).await;
    let account = user_by_access_token(&state, &access_token).await;

    assert_eq!(account.username, username);
    assert_eq!(account.acct, username);
}
