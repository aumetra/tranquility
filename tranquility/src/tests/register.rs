use {
    super::{init_db, init_state, test_config},
    crate::state::State,
    warp::{
        hyper::{body, StatusCode},
        Reply,
    },
};

#[tokio::test]
async fn closed_registrations() {
    let mut config = test_config();
    config.instance.closed_registrations = true;
    let db_pool = init_db().await;

    let state = State::new(config, db_pool);
    crate::state::init_raw(state);

    let register_endpoint = crate::api::register::routes();

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

#[tokio::test]
async fn register_endpoint() {
    init_state().await;
    let register_endpoint = crate::api::register::routes();

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
    assert_eq!(body_data, b"Account created" as &[u8]);
}
