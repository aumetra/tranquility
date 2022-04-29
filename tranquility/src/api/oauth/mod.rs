use crate::consts::cors::OAUTH_TOKEN_ALLOWED_METHODS;
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

pub fn routes() -> Router {
    let token_router = Router::new()
        .route("/token", post(token::token))
        .layer(CorsLayer::very_permissive().allow_methods(OAUTH_TOKEN_ALLOWED_METHODS.to_vec()));

    let authorize_router = Router::new()
        .route("/authorize", get(authorize::get))
        .route("/authorize", post(authorize::post));

    let router = Router::new().merge(authorize_router).merge(token_router);
    Router::new().nest("/oauth", router)
}

pub mod authorize;
pub mod token;
