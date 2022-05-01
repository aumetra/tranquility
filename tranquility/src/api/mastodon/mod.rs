use crate::{
    consts::cors::API_ALLOWED_METHODS,
    database::{Actor, OAuthToken},
    error::Error,
    state::ArcState,
};
use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    Router,
};
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use once_cell::sync::Lazy;
use std::ops::Deref;
use tower_http::cors::CorsLayer;
use tranquility_types::mastodon::App;

static DEFAULT_APPLICATION: Lazy<App> = Lazy::new(|| App {
    name: "Web".into(),
    ..App::default()
});

/// Authorisation extractor
///
/// It takes the `Authorization` header and tries to decodes it as an `Bearer` authorisation.  
/// Then it fetches the actor associated with the token
pub struct Authorisation(pub Actor);

impl Deref for Authorisation {
    type Target = Actor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for Authorisation
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let credentials = req
            .headers()
            .typed_get::<Authorization<Bearer>>()
            .ok_or(Error::Unauthorized)?;
        let token = credentials.token();

        let state = req
            .extensions()
            .get::<ArcState>()
            .expect("[Bug] Missing state in extensions");

        let access_token = OAuthToken::by_access_token(&state.db_pool, token).await?;
        let actor = Actor::get(&state.db_pool, access_token.actor_id).await?;

        Ok(Self(actor))
    }
}

pub fn routes() -> Router {
    let v1_router = Router::new()
        .merge(accounts::routes())
        .merge(apps::routes())
        .merge(statuses::routes())
        .merge(instance::routes());

    Router::new()
        .nest("/api/v1", v1_router)
        .layer(CorsLayer::permissive().allow_methods(API_ALLOWED_METHODS.to_vec()))
}

pub mod accounts;
pub mod apps;
pub mod convert;
pub mod instance;
pub mod statuses;
