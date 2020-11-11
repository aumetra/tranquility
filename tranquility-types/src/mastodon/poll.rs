// To avoid name collision between the `Option` and the `PollOption` types
#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct PollOption {
    pub title: String,
    pub votes_count: i64,
}
