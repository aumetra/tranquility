use {
    crate::{
        consts::cors::OAUTH_TOKEN_ALLOWED_METHODS, limit_body_size, state::ArcState,
        util::construct_cors,
    },
    askama::Template,
    once_cell::sync::Lazy,
    tranquility_ratelimit::{ratelimit, Configuration as RatelimitConfig},
    warp::{Filter, Rejection, Reply},
};

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

fn authorize_route<F>(
    state: &ArcState,
    ratelimit_filter: F,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    F: Filter<Extract = (), Error = Rejection> + Clone + Send + Sync + 'static,
{
    let state = crate::state::filter(state);

    let get = warp::get().and_then(authorize::get);

    // Ratelimit only the logic
    let post = warp::post()
        .and(state)
        .and(warp::body::form())
        .and(warp::query())
        .and_then(authorize::post)
        .with(ratelimit!(from_filter: ratelimit_filter));
    // Restrict the body size
    let post = limit_body_size!(post);

    warp::path!("oauth" / "authorize").and(get.or(post))
}

fn token_route<F>(
    state: &ArcState,
    ratelimit_filter: F,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    F: Filter<Extract = (), Error = Rejection> + Clone + Send + Sync + 'static,
{
    let state = crate::state::filter(state);

    // Enable CORS for the token endpoint
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb
    let cors = construct_cors(OAUTH_TOKEN_ALLOWED_METHODS);
    let token_path = warp::path!("oauth" / "token");

    // Ratelimit only the logic
    let token_logic = warp::post()
        .and(state)
        .and(warp::body::form())
        .and_then(token::token)
        .with(ratelimit!(from_filter: ratelimit_filter));
    // Restrict the body size
    let token_logic = limit_body_size!(token_logic);

    token_path.and(token_logic).with(cors)
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let ratelimit_config = RatelimitConfig::new()
        .active(state.config.ratelimit.active)
        .burst_quota(state.config.ratelimit.authentication_quota);
    let ratelimit_filter =
        ratelimit(ratelimit_config).expect("Couldn't construct a ratelimit filter");

    let authorize = authorize_route(state, ratelimit_filter.clone());
    let token = token_route(state, ratelimit_filter);

    authorize.or(token)
}

pub mod authorize;
pub mod token;
