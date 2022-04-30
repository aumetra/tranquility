use crate::state::ArcState;
use axum::{extract::connect_info::IntoMakeServiceWithConnectInfo, Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    io,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

/// Construct the combined router
pub fn create_router_make_service(
    state: &ArcState,
) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let router = Router::new()
        .merge(crate::activitypub::routes())
        .merge(crate::api::routes(state))
        .merge(crate::well_known::routes())
        .layer(Extension(Arc::clone(state)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    #[cfg(feature = "email")]
    let router = router.merge(crate::email::routes());

    router.into_make_service_with_connect_info()
}

/// Combine all routers and start the webserver
pub async fn run(state: ArcState) -> io::Result<()> {
    let router_service = create_router_make_service(&state);
    let interface = state.config.server.interface.parse::<IpAddr>().unwrap();
    let addr = (interface, state.config.server.port);

    if state.config.tls.serve_tls_directly {
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
