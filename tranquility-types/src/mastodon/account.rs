use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing an [Mastodon account](https://docs.joinmastodon.org/entities/account/)
pub struct Account {
    pub id: String,

    pub username: String,
    pub acct: String,
    pub display_name: String,

    pub locked: bool,
    pub bot: bool,

    pub created_at: String,
    pub note: String,
    pub url: String,

    pub avatar: String,
    pub avatar_static: String,

    pub header: String,
    pub header_static: String,

    pub followers_count: i64,
    pub following_count: i64,
    pub statuses_count: i64,

    pub last_status_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<super::Source>,

    pub emojis: Vec<super::Emoji>,
    pub fields: Vec<super::Field>,
}

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the answer to a successful follow
pub struct FollowResponse {
    pub id: String,

    pub showing_reblogs: bool,
    pub notifying: bool,
    pub requested: bool,
    pub endorsed: bool,

    pub following: bool,
    pub followed_by: bool,

    pub blocking: bool,
    pub blocked_by: bool,
    pub domain_blocking: bool,

    pub muting: bool,
    pub muting_notifications: bool,
}
