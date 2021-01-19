use {
    tranquility_ratelimit::Configuration,
    warp::{Filter, Rejection, Reply},
};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let mastodon_api = mastodon::routes();

    let oauth = oauth::routes();

    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(register::register);

    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.registration_quota);

    let register =
        tranquility_ratelimit::ratelimit!(filter => register, config => ratelimit_config).unwrap();

    mastodon_api.or(oauth).or(register)
}

pub mod mastodon;
pub mod oauth;
pub mod register;
