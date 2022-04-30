use axum::Router;
use cfg_if::cfg_if;

use crate::state::State;

pub fn routes(state: &State) -> Router {
    let router = Router::new()
        .merge(oauth::routes(state))
        .merge(register::routes(state));

    cfg_if! {
        if #[cfg(feature = "mastodon-api")] {
            router.merge(mastodon::routes())
        } else {
            router
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
