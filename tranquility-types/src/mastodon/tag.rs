use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing a [Mastodon tag](https://docs.joinmastodon.org/entities/tag/)
pub struct Tag {
    pub name: String,
    pub url: String,

    pub history: Option<History>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing a [Mastodon history](https://docs.joinmastodon.org/entities/history/)
pub struct History {
    pub day: String,
    pub uses: String,
    pub accounts: String,
}
