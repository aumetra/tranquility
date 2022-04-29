use crate::{database::Actor as DbActor, error::Error, state::ArcState, util::HTTP_CLIENT};
use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tranquility_types::{
    activitypub::Actor,
    webfinger::{Link, Resource},
};

// Keeping this for future use
pub async fn fetch_actor(
    state: &ArcState,
    username: &str,
    domain: &str,
) -> Result<(Actor, DbActor), Error> {
    let resource = format!("acct:{}@{}", username, domain);
    let url = format!(
        "https://{}/.well-known/webfinger?resource={}",
        domain, resource
    );

    let request = HTTP_CLIENT
        .get(&url)
        .header("Accept", "application/jrd+json")
        .build()?;
    let resource: Resource = HTTP_CLIENT.execute(request).await?.json().await?;

    let actor_url = resource
        .links
        .iter()
        .find(|link| link.rel == "self")
        .ok_or(Error::UnexpectedWebfingerResource)?;

    crate::activitypub::fetcher::fetch_actor(state, &actor_url.href).await
}

#[derive(Deserialize)]
/// Query struct for a webfinger request
pub struct QueryParams {
    resource: String,
}

pub async fn webfinger(
    Extension(state): Extension<ArcState>,
    Query(QueryParams { resource }): Query<QueryParams>,
) -> Result<impl IntoResponse, Error> {
    let mut resource_tokens = resource.trim_start_matches("acct:").split('@');
    let username = resource_tokens.next().ok_or(Error::InvalidRequest)?;

    if resource_tokens.next().ok_or(Error::InvalidRequest)? != state.config.instance.domain {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let actor_db = DbActor::by_username_local(&state.db_pool, username).await?;
    let actor: Actor = serde_json::from_value(actor_db.actor)?;

    let link = Link {
        rel: "self".into(),
        r#type: Some("application/activity+json".into()),
        href: actor.id.clone(),
        ..Link::default()
    };
    let resource = Resource {
        subject: resource,

        aliases: vec![actor.id],

        links: vec![link],
        ..Resource::default()
    };

    Ok(([("Content-Type", "application/jrd+json")], Json(resource)).into_response())
}

pub fn routes() -> Router {
    Router::new()
        .route("/webfinger", get(webfinger))
        .layer(CorsLayer::very_permissive())
}
