use {crate::state::ArcState, std::net::IpAddr, warp::Filter};

/// Combine all route filters and start a warp server
pub async fn run(state: ArcState) {
    let logging = warp::trace::request();

    let activitypub = crate::activitypub::routes::routes(&state);
    let api = crate::api::routes(&state);
    let well_known = crate::well_known::routes(&state);

    let routes = activitypub.or(api).or(well_known);

    #[cfg(feature = "email")]
    let routes = {
        let email = crate::email::routes(&state);
        routes.or(email)
    };

    let routes = routes.with(logging).recover(crate::error::recover);

    let server = warp::serve(routes);

    let config = &state.config;

    let interface = config.server.interface.parse::<IpAddr>().unwrap();
    let addr = (interface, config.server.port);

    if config.tls.serve_tls_directly {
        server
            .tls()
            .cert_path(&config.tls.certificate)
            .key_path(&config.tls.secret_key)
            .run(addr)
            .await
    } else {
        server.run(addr).await
    }
}
