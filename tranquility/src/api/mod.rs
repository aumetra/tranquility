use {
    tranquility_ratelimit::{ratelimit, Configuration},
    warp::{Filter, Rejection, Reply},
};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    #[cfg(feature = "mastodon-api")]
    let mastodon_api = mastodon::routes();

    let oauth = oauth::routes();

    let register_path = warp::path!("api" / "tranquility" / "v1" / "register");

    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.registration_quota);

    // Ratelimit only the logic
    let ratelimit_wrapper =
        ratelimit!(from_config: ratelimit_config).expect("Couldn't construct ratelimit wrapper");
    let register_logic = warp::post()
        .and(warp::body::form())
        .and_then(register::register)
        .with(ratelimit_wrapper);

    let register = register_path.and(register_logic);

    #[cfg(feature = "mastodon-api")]
    {
        mastodon_api.or(oauth).or(register)
    }
    #[cfg(not(feature = "mastodon-api"))]
    {
        oauth.or(register)
    }
}

#[cfg(feature = "mastodon-api")]
pub mod mastodon;
pub mod oauth;
pub mod register;
