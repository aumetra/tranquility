use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing a [Mastodon card](https://docs.joinmastodon.org/entities/card/)
pub struct Card {
    pub r#type: String,

    pub url: String,
    pub title: String,
    pub description: String,

    pub author_name: String,
    pub author_url: String,
    pub provider_name: String,
    pub html: String,

    pub width: i64,
    pub height: i64,

    pub image: String,
    pub embed_url: String,
    pub blurhash: String,
}
