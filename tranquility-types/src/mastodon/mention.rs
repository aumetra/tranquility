use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
/// Struct representing a [Mastodon mention](https://docs.joinmastodon.org/entities/mention/)
pub struct Mention {
    pub id: String,

    pub username: String,
    pub acct: String,

    pub url: String,
}
