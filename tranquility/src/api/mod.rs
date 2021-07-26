use {
    cfg_if::cfg_if,
    warp::{Filter, Rejection, Reply},
};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let oauth = oauth::routes();
    let register = register::routes();

    let auth_api = oauth.or(register);

    cfg_if! {
        if #[cfg(feature = "mastodon-api")] {
            mastodon::routes().or(auth_api)
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
                use pulldown_cmark::{html, Options, Parser};

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
