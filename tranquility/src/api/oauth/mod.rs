use {
    askama::Template,
    once_cell::sync::Lazy,
    tranquility_ratelimit::Configuration,
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
    let authorize = {
        let get = warp::get().and_then(authorize::get);
        let post = warp::post()
            .and(warp::body::form())
            .and(warp::query())
            .and_then(authorize::post);

        warp::path!("oauth" / "authorize").and(get.or(post))
    };
    let token = warp::path!("oauth" / "token")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(token::token);

    let oauth_routes = authorize.or(token);

    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.authentication_quota);

    tranquility_ratelimit::ratelimit!(filter => oauth_routes, config => ratelimit_config).unwrap()
}

pub mod authorize;
pub mod token;
