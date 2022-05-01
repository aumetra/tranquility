use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
/// Struct representing a [Mastodon status](https://docs.joinmastodon.org/entities/status/)
pub struct Status {
    pub id: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,

    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub language: String,

    pub uri: String,
    pub url: String,

    pub replies_count: i64,
    pub reblogs_count: i64,
    pub favourites_count: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reblogged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,

    pub content: String,
    pub reblog: Option<Box<Status>>,

    pub application: super::App,
    pub account: super::Account,

    pub media_attachments: Vec<super::Attachment>,
    pub mentions: Vec<super::Mention>,
    pub tags: Vec<super::Tag>,

    pub card: Option<super::Card>,
    pub poll: Option<super::Poll>,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: OffsetDateTime::now_utc(),
            in_reply_to_id: Option::default(),
            in_reply_to_account_id: Option::default(),
            sensitive: bool::default(),
            spoiler_text: String::default(),
            visibility: String::default(),
            language: String::default(),
            uri: String::default(),
            url: String::default(),
            replies_count: i64::default(),
            reblogs_count: i64::default(),
            favourites_count: i64::default(),
            favourited: Option::default(),
            reblogged: Option::default(),
            muted: Option::default(),
            bookmarked: Option::default(),
            content: String::default(),
            reblog: Option::default(),
            application: super::App::default(),
            account: super::Account::default(),
            media_attachments: Vec::default(),
            mentions: Vec::default(),
            tags: Vec::default(),
            card: Option::default(),
            poll: Option::default(),
        }
    }
}
