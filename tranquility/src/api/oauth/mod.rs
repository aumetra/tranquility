use {
    askama::Template,
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

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
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

    authorize.or(token)
}

pub mod authorize;
pub mod token;
