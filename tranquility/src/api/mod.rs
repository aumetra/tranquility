use {
    tranquility_ratelimit::Configuration,
    warp::{Filter, Rejection, Reply},
};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let mastodon_api = mastodon::routes();

    let oauth = oauth::routes();

    let register_path = warp::path!("api" / "register");
    let register_logic = warp::post()
        .and(warp::body::form())
        .and_then(register::register);

    let config = crate::config::get();
    let ratelimit_config = Configuration::new()
        .active(config.ratelimit.active)
        .burst_quota(config.ratelimit.registration_quota);

    // Ratelimit only the logic
    let register_logic =
        tranquility_ratelimit::ratelimit!(filter => register_logic, config => ratelimit_config)
            .unwrap();
    let register = register_path.and(register_logic);

    mastodon_api.or(oauth).or(register)
}

pub mod mastodon;
pub mod oauth;
pub mod register;
