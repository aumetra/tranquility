use super::{Attachment, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    #[serde(rename = "@context")]
    pub _context: Value,

    pub id: String,
    // (Should) always equal "Person"
    pub r#type: String,

    // Display name
    pub name: String,
    // Unique username
    pub username: String,

    pub summary: String,
    // In case you mention someone in your summary
    pub tag: Vec<Tag>,
    // Profile picture
    pub icon: Option<Attachment>,
    // Header image
    pub image: Option<Attachment>,

    pub manually_approves_followers: bool,

    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub public_key: PublicKey,
}
