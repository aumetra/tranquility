use crate::tests::{start_test_server, test_state};
use http::StatusCode;

#[tokio::test]
async fn closed_registrations() {
    let mut state = test_state().await;
    state.config.instance.closed_registrations = true;
    let test_client = start_test_server(state);

    let test_response = test_client
        .post(
            "/api/tranquility/v1/register",
            "username=test&email=test@example.com&password=1234.",
        )
        .await
        .expect("Failed to send registration request");
    assert_eq!(test_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn register_endpoint() {
    let state = test_state().await;
    let test_client = start_test_server(state);

    let test_response = test_client
        .post(
            "/api/tranquility/v1/register",
            "username=test&email=test@example.com&password=test1234.",
        )
        .await
        .expect("Failed to send registration request");
    assert_eq!(test_response.status(), StatusCode::CREATED);

    let body_data = test_response
        .text()
        .await
        .expect("Failed to get text response");
    assert_eq!(body_data, "Account created");
}
