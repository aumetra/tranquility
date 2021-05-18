// To avoid name collision between the `Option` and the `PollOption` types
#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
/// Struct representing a [Mastodon poll](https://docs.joinmastodon.org/entities/poll/)
pub struct Poll {
    pub id: String,

    pub expires_at: String,
    pub expired: bool,

    pub multiple: bool,
    pub votes_count: i64,
    pub voters_count: Option<i64>,

    pub own_votes: Option<Vec<i64>>,

    pub options: Vec<PollOption>,
    pub emojis: Vec<super::Emoji>,
}

#[derive(Default, Deserialize, Serialize)]
/// Struct representing on option of a [Poll]
pub struct PollOption {
    pub title: String,
    pub votes_count: i64,
}
