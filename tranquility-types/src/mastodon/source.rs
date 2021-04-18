use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing a [Mastodon source](https://docs.joinmastodon.org/entities/source/)
pub struct Source {
    pub privacy: String,
    pub sensitive: bool,
    pub language: String,

    pub note: String,
    pub fields: Vec<super::Field>,
    pub follow_requests_count: i64,
}
