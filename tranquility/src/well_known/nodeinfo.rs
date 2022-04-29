use crate::{
    consts::{SOFTWARE_NAME, VERSION},
    state::ArcState,
};
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use tranquility_types::nodeinfo::{Link, LinkCollection, Nodeinfo, Services, Software, Usage};

#[allow(clippy::unused_async)]
async fn nodeinfo(Extension(state): Extension<ArcState>) -> impl IntoResponse {
    let info = Nodeinfo {
        protocols: vec!["activitypub".into()],
        software: Software {
            name: SOFTWARE_NAME.into(),
            version: VERSION.into(),
            ..Software::default()
        },
        services: Services {
            inbound: Vec::new(),
            outbound: Vec::new(),
        },
        open_registrations: !state.config.instance.closed_registrations,
        usage: Usage::default(),
        ..Nodeinfo::default()
    };

    Json(info)
}

#[allow(clippy::unused_async)]
async fn nodeinfo_links(Extension(state): Extension<ArcState>) -> impl IntoResponse {
    let entity_link = format!(
        "https://{}/.well-known/nodeinfo/2.1",
        state.config.instance.domain
    );

    let link = Link::new(entity_link);
    let link_collection = LinkCollection { links: vec![link] };

    Json(link_collection)
}

pub fn routes() -> Router {
    Router::new()
        .route("/nodeinfo", get(nodeinfo_links))
        .route("/nodeinfo/2.1", get(nodeinfo))
}
