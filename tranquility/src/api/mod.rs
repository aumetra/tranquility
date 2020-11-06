use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let mastodon_api = mastodon::routes();

    let oauth = oauth::routes();

    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(register::register);

    mastodon_api.or(oauth).or(register)
}

pub mod mastodon;
pub mod oauth;
pub mod register;
