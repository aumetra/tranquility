use warp::Filter;

pub async fn run() {
    let logging = warp::log("");

    let activitypub = crate::activitypub::routes::routes();
    let api = crate::api::routes();

    let routes = activitypub
        .or(api)
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
