use {
    super::Account,
    serde::{Deserialize, Serialize},
};

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the `stats` field of an [Instance]
pub struct Stats {
    pub user_count: u64,
    pub status_count: u64,
    pub domain_count: u64,
}

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the `urls` field of an [Instance]
pub struct Urls {
    pub streaming_api: String,
}

#[derive(Default, Deserialize, PartialEq, Serialize)]
/// Struct representing an [Mastodon instance](https://docs.joinmastodon.org/entities/instance/)
pub struct Instance {
    pub uri: String,
    pub title: String,
    pub short_description: Option<String>,
    pub description: String,
    pub email: Option<String>,
    pub version: String,
    pub urls: Urls,
    pub stats: Stats,
    pub thumbnail: Option<String>,
    pub language: Vec<String>,
    pub registrations: bool,
    pub approval_required: bool,
    pub invites_enabled: bool,
    pub contact_account: Option<Account>,
}
