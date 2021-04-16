use {
    crate::{
        consts::{MAX_BODY_SIZE, MB_BYTES},
        state::ArcState,
    },
    warp::{Filter, Rejection, Reply},
};

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    #[cfg(feature = "mastodon-api")]
    let mastodon_api = {
        let limit = (state.config.instance.upload_limit as u64) * MB_BYTES;
        warp::body::content_length_limit(limit).and(mastodon::routes(state))
    };

    let oauth = oauth::routes(state);
    let register = register::routes(state);
    let auth_api = warp::body::content_length_limit(MAX_BODY_SIZE).and(oauth.or(register));

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
