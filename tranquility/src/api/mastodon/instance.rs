use crate::{consts::VERSION, error::Error, state::ArcState};
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use tranquility_types::mastodon::{
    instance::{Stats, Urls},
    Instance,
};

#[allow(clippy::unused_async)]
async fn instance(Extension(state): Extension<ArcState>) -> Result<impl IntoResponse, Error> {
    let streaming_api = format!("wss://{}", state.config.instance.domain);

    let instance = Instance {
        version: VERSION.into(),
        title: state.config.instance.domain.clone(),
        uri: state.config.instance.domain.clone(),
        short_description: None,
        description: state.config.instance.description.clone(),

        urls: Urls { streaming_api },
        stats: Stats { ..Stats::default() },

        registrations: !state.config.instance.closed_registrations,
        invites_enabled: false,
        approval_required: false,

        email: None,
        contact_account: None,

        ..Instance::default()
    };

    Ok(Json(instance))
}

pub fn routes() -> Router {
    Router::new().route("/instance", get(instance))
}
