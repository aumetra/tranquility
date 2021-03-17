use {
    crate::state::ArcState,
    warp::{Filter, Rejection, Reply},
};

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    #[cfg(feature = "mastodon-api")]
    let mastodon_api = mastodon::routes(state);

    let oauth = oauth::routes(state);
    let register = register::routes(state);

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
