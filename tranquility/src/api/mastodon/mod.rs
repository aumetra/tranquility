use axum::Router;
use tower_http::cors::CorsLayer;

use {
    crate::{
        consts::cors::API_ALLOWED_METHODS,
        database::{Actor, OAuthToken},
        error::Error,
        state::ArcState,
    },
    headers::authorization::{Bearer, Credentials},
    once_cell::sync::Lazy,
    tranquility_types::mastodon::App,
    warp::{
        hyper::header::{HeaderValue, AUTHORIZATION},
        reject::MissingHeader,
        Filter, Rejection, Reply,
    },
};

static DEFAULT_APPLICATION: Lazy<App> = Lazy::new(|| App {
    name: "Web".into(),
    ..App::default()
});

/// Same as [`authorise_user`] but makes the bearer token optional
pub fn authorisation_optional(
    state: &ArcState,
) -> impl Filter<Extract = (Option<Actor>,), Error = Rejection> + Clone {
    let or_none_fn = |error: Rejection| async move {
        if error.find::<MissingHeader>().is_some() {
            Ok((None,))
        } else {
            Err(error)
        }
    };

    authorisation_required(state).map(Some).or_else(or_none_fn)
}

/// Parses a `HeaderValue` as the contents of an authorization header with bearer token contents
/// and attempts to fetch the user from the database
async fn authorise_user(
    state: ArcState,
    authorization_header: HeaderValue,
) -> Result<Actor, Rejection> {
    let credentials = Bearer::decode(&authorization_header).ok_or(Error::Unauthorized)?;
    let token = credentials.token();

    let access_token = OAuthToken::by_access_token(&state.db_pool, token).await?;
    let actor = Actor::get(&state.db_pool, access_token.actor_id).await?;

    Ok(actor)
}

/// Filter that gets the user associated with the bearer token from the database
///
/// Rejects if the user cannot be found
pub fn authorisation_required(
    state: &ArcState,
) -> impl Filter<Extract = (Actor,), Error = Rejection> + Clone {
    crate::state::filter(state)
        .and(warp::header::value(AUTHORIZATION.as_ref()))
        .and_then(authorise_user)
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Enable CORS for all API endpoints
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb

    let v1_router = Router::new()
        .merge(accounts::routes())
        .merge(apps::routes())
        .merge(statuses::routes())
        .merge(instance::routes());

    Router::new()
        .nest("/api/v1", v1_router)
        .layer(CorsLayer::new().allow_methods(API_ALLOWED_METHODS))
}

pub mod accounts;
pub mod apps;
pub mod convert;
pub mod instance;
pub mod statuses;
