use {
    askama::Template,
    tranquility_ratelimit::Configuration,
    warp::{Filter, Rejection, Reply},
};

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

    // Limit the OAuth routes to 50 requests per hour
    // TODO: Make configurable
    tranquility_ratelimit::ratelimit!(filter => oauth_routes, config => Configuration::default())
        .unwrap()
}

pub mod authorize;
pub mod token;
