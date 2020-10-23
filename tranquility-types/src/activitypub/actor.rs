use super::{Attachment, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
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
    #[serde(rename = "preferredUsername")]
    pub username: String,

    pub summary: String,
    // In case you mention someone in your summary
    #[serde(default)]
    pub tag: Vec<Tag>,
    // Profile picture
    pub icon: Option<Attachment>,
    // Header image
    pub image: Option<Attachment>,

    #[serde(default)]
    pub manually_approves_followers: bool,

    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub public_key: PublicKey,
}

pub fn create(user_id: &str, username: &str, public_key_pem: String, domain: &str) -> Actor {
    let prefix = format!("https://{}", domain);
    let id = format!("{}/users/{}", prefix, user_id);

    let inbox = format!("{}/inbox", id);
    let outbox = format!("{}/outbox", id);

    let followers = format!("{}/followers", id);
    let following = format!("{}/following", id);

    let key_id = format!("{}#main-key", id);

    let public_key = PublicKey {
        id: key_id,
        owner: id.clone(),
        public_key_pem,
    };

    Actor {
        _context: super::context_field(),

        id,
        r#type: "Person".into(),

        username: username.into(),

        inbox,
        outbox,

        followers,
        following,

        public_key,
        ..Default::default()
    }
}
