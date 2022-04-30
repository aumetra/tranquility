use crate::state::ArcState;
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    io,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

/// Combine all route filters and start a warp server
pub async fn run(state: ArcState) -> io::Result<()> {
    let router = Router::new()
        .merge(crate::activitypub::routes())
        .merge(crate::api::routes(&state))
        .merge(crate::well_known::routes())
        .layer(Extension(Arc::clone(&state)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    #[cfg(feature = "email")]
    let router = router.merge(crate::email::routes());

    let router_service = router.into_make_service_with_connect_info::<SocketAddr>();

    let config = &state.config;

    let interface = config.server.interface.parse::<IpAddr>().unwrap();
    let addr = (interface, config.server.port);

    if config.tls.serve_tls_directly {
        let config = RustlsConfig::from_pem_file(
            &state.config.tls.certificate,
            &state.config.tls.secret_key,
        )
        .await?;

        axum_server::bind_rustls(addr.into(), config)
            .serve(router_service)
            .await?;
    } else {
        axum_server::bind(addr.into()).serve(router_service).await?;
    }

    Ok(())
}
