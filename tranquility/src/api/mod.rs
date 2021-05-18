use {
    crate::{
        consts::{MAX_BODY_SIZE, MB_BYTES},
        state::ArcState,
    },
    cfg_if::cfg_if,
    warp::{Filter, Rejection, Reply},
};

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let oauth = oauth::routes(state);
    let register = register::routes(state);

    let auth_api = warp::body::content_length_limit(MAX_BODY_SIZE).and(oauth.or(register));

    cfg_if! {
        if #[cfg(feature = "mastodon-api")] {
            let mastodon_api = {
                let limit = (state.config.instance.upload_limit as u64) * MB_BYTES;
                warp::body::content_length_limit(limit).and(mastodon::routes(state))
            };

            mastodon_api.or(auth_api)
        } else {
            auth_api
        }
    }
}

cfg_if! {
    if #[cfg(feature = "markdown")] {
        pub trait ParseMarkdown {
            fn parse_markdown(&mut self);
        }

        impl ParseMarkdown for tranquility_types::activitypub::Object {
            fn parse_markdown(&mut self) {
                use markdown::{html, Options, Parser};

                let content = self.content.clone();
                let parser = Parser::new_ext(&content, Options::all());

                html::push_html(&mut self.content, parser);
            }
        }
    }
}

#[cfg(feature = "mastodon-api")]
pub mod mastodon;

pub mod oauth;
pub mod register;
