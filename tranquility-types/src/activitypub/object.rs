use super::{Attachment, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "@context")]
    pub _context: Value,

    pub id: String,
    pub r#type: String,
    pub actor: String,
    pub attributed_to: String,

    pub content: String,
    pub published: String,
    pub sensitive: bool,

    pub attachment: Vec<Attachment>,
    pub tag: Vec<Tag>,

    pub to: Vec<String>,
    pub cc: Vec<String>,
}
