use {crate::state::ArcState, warp::Filter};

pub async fn run(state: ArcState) {
    let logging = warp::trace::request();

    let activitypub = crate::activitypub::routes::routes(&state);
    let api = crate::api::routes(&state);
    let webfinger = crate::webfinger::routes(&state);

    let routes = activitypub
        .or(api)
        .or(webfinger)
        .with(logging)
        .recover(crate::error::recover);

    let server = warp::serve(routes);

    let config = &state.config;
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
