use crate::{consts::cors::OAUTH_TOKEN_ALLOWED_METHODS, ratelimit_layer, state::State};
use askama::Template;
use axum::{
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use tower_http::cors::CorsLayer;

// This form has no fields. Rendering it every time is a waste
static AUTHORIZE_FORM: Lazy<String> = Lazy::new(|| AuthorizeFormTemplate.render().unwrap());

#[derive(Template)]
#[template(path = "oauth/authorize.html")]
struct AuthorizeFormTemplate;

#[derive(Template)]
#[template(path = "oauth/token.html")]
struct TokenTemplate {
    token: String,
}

pub fn routes(state: &State) -> Router {
    let token_router = Router::new()
        .route("/token", post(token::token))
        .layer(CorsLayer::very_permissive().allow_methods(OAUTH_TOKEN_ALLOWED_METHODS.to_vec()));

    let authorize_router =
        Router::new().route("/authorize", get(authorize::get).post(authorize::post));

    let router = Router::new().merge(authorize_router).merge(token_router);
    Router::new()
        .nest("/oauth", router)
        .route_layer(ratelimit_layer!(
            state.config.ratelimit.active,
            state.config.ratelimit.authentication_quota,
        ))
}

pub mod authorize;
pub mod token;
