use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
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
