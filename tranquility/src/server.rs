use warp::Filter;

pub async fn run() {
    let logging = warp::log("");
    let activitypub_header_check =
        warp::header::exact_ignore_case("accept", "application/activity+json").or(
            warp::header::exact_ignore_case(
                "accept",
                "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
            ),
        );

    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(crate::routes::register::register);

    let users = warp::path!("users" / String)
        .and(warp::get())
        .and(activitypub_header_check)
        .map(|uuid, _| uuid)
        .and_then(crate::routes::users::get_actor);

    let inbox = warp::path!("users" / String / "inbox")
        .and(warp::post())
        .and(
            warp::header::value("authorization")
                .or(warp::header::value("signature"))
                .unify(),
        )
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::raw())
        .and(warp::header::headers_cloned())
        .and(warp::body::json())
        .and_then(crate::routes::inbox::verify_request)
        .and_then(crate::routes::inbox::inbox);

    let routes = register
        .or(users)
        .or(inbox)
        .with(logging)
        .recover(crate::error::recover);

    let config = crate::config::get();
    let server = warp::serve(routes);

    if !config.tls.reverse_proxy {
        server
            .tls()
            .cert_path(&config.tls.certificate)
            .key_path(&config.tls.secret_key)
            .run(([0, 0, 0, 0], config.port))
            .await
    } else {
        server.run(([0, 0, 0, 0], config.port)).await
    }
}
