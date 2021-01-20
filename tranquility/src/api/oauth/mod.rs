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
    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.authentication_quota);
    let ratelimit_filter = tranquility_ratelimit::ratelimit(ratelimit_config).unwrap();

    let authorize = {
        let get = warp::get().and_then(authorize::get);

        let ratelimit_filter = ratelimit_filter.clone();
        let post = warp::post()
            .and(warp::body::form())
            .and(warp::query())
            .and_then(authorize::post);
        // Ratelimit only the logic
        let post = tranquility_ratelimit::custom_ratelimit!(filter => post, ratelimit_filter => ratelimit_filter);

        warp::path!("oauth" / "authorize").and(get.or(post))
    };
    let token_path = warp::path!("oauth" / "token");

    // Ratelimit only the logic
    let token_logic = warp::post().and(warp::body::form()).and_then(token::token);
    let token_logic = tranquility_ratelimit::custom_ratelimit!(filter => token_logic, ratelimit_filter => ratelimit_filter);

    let token = token_path.and(token_logic);

    authorize.or(token)
}

pub mod authorize;
pub mod token;
