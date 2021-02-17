use {
    askama::Template,
    once_cell::sync::Lazy,
    tranquility_ratelimit::{ratelimit, Configuration},
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

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.authentication_quota);
    let ratelimit_filter =
        ratelimit(ratelimit_config).expect("Couldn't construct a ratelimit filter");

    let authorize = {
        let get = warp::get().and_then(authorize::get);

        // Ratelimit only the logic
        let post = warp::post()
            .and(warp::body::form())
            .and(warp::query())
            .and_then(authorize::post)
            .with(ratelimit!(from_filter: ratelimit_filter.clone()));

        warp::path!("oauth" / "authorize").and(get.or(post))
    };
    let token_path = warp::path!("oauth" / "token");

    // Ratelimit only the logic
    let token_logic = warp::post()
        .and(warp::body::form())
        .and_then(token::token)
        .with(ratelimit!(from_filter: ratelimit_filter));

    let token = token_path.and(token_logic);

    authorize.or(token)
}

pub mod authorize;
pub mod token;
