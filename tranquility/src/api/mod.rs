use {
    crate::config::ArcConfig,
    warp::{Filter, Rejection, Reply},
};

pub fn routes(
    config: ArcConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    #[cfg(feature = "mastodon-api")]
    let mastodon_api = mastodon::routes(config.clone());

    let oauth = oauth::routes(&config);
    let register = register::routes(config);

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
