use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
/// Struct representing an [Mastodon application](https://docs.joinmastodon.org/entities/application/)
pub struct App {
    pub id: String,

    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: String,

    pub client_id: String,
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vapid_key: Option<String>,
}
