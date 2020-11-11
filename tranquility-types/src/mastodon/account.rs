use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
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
