use {
    crate::tests::test_state,
    std::sync::Arc,
    warp::{hyper::body, Reply},
};

const INSTANCE_ENDPOINT_RESPONSE: &str = concat!(
    r#"{"uri":"tranquility.example.com","title":"tranquility.example.com","short_description":null,"description":"Tranquility instance","email":null,"version":""#,
    env!("CARGO_PKG_VERSION"),
    r#"","urls":{"streaming_api":"wss://tranquility.example.com"},"stats":{"user_count":0,"status_count":0,"domain_count":0},"thumbnail":null,"language":[],"registrations":true,"approval_required":false,"invites_enabled":false,"contact_account":null}"#
);

#[tokio::test]
/// Test that the instance info endpoint returns correct data
async fn instance_info() {
    let state = Arc::new(test_state().await);

    let mastodon_api = crate::api::mastodon::routes(&state);
    let response = warp::test::request()
        .path("/api/v1/instance")
        .filter(&mastodon_api)
        .await
        .expect("Unsuccessful request")
        .into_response();
    assert!(response.status().is_success());

    let data = body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(data, INSTANCE_ENDPOINT_RESPONSE);
}
