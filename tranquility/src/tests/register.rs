use {
    super::{init_db, test_config},
    std::sync::Arc,
    warp::{
        hyper::{body, StatusCode},
        Reply,
    },
};

#[tokio::test(flavor = "multi_thread")]
async fn register_endpoint() {
    init_db().await;

    let config = Arc::new(test_config());

    let register_endpoint = crate::api::register::routes(config);

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

#[tokio::test]
async fn closed_registrations() {
    init_db().await;

    let mut config = test_config();
    config.instance.closed_registrations = true;

    let register_endpoint = crate::api::register::routes(Arc::new(config));

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
