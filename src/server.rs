use warp::Filter;

pub async fn run() {
    let logging = warp::log("");
    let routes = warp::any()
        .map(|| format!("Running Tranquility v{}", env!("CARGO_PKG_VERSION")))
        .with(logging);

    let config = crate::config::CONFIGURATION.get().unwrap();
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
