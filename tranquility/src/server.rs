use warp::Filter;

pub async fn run() {
    let logging = warp::trace::request();

    let activitypub = crate::activitypub::routes::routes();
    let api = crate::api::routes();
    let webfinger = crate::webfinger::routes();

    let routes = activitypub
        .or(api)
        .or(webfinger)
        .with(logging)
        .recover(crate::error::recover);

    let config = crate::config::get();
    let server = warp::serve(routes);

    if config.tls.serve_tls_directly {
        server
            .tls()
            .cert_path(&config.tls.certificate)
            .key_path(&config.tls.secret_key)
            .run(([0, 0, 0, 0], config.server.port))
            .await
    } else {
        server.run(([0, 0, 0, 0], config.server.port)).await
    }
}
