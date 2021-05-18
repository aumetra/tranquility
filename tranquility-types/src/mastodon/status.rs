use {
    serde::{Deserialize, Serialize},
    std::boxed::Box,
};

#[derive(Default, Deserialize, Serialize)]
/// Struct representing a [Mastodon status](https://docs.joinmastodon.org/entities/status/)
pub struct Status {
    pub id: String,
    pub created_at: String,

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
