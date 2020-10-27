use super::{Attachment, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(default = "super::context_field", rename = "@context")]
    pub context: Value,

    pub id: String,
    pub r#type: String,

    pub attributed_to: String,

    pub content: String,
    pub published: String,
    #[serde(default)]
    pub sensitive: bool,

    #[serde(default)]
    pub attachment: Vec<Attachment>,
    #[serde(default)]
    pub tag: Vec<Tag>,

    pub to: Vec<String>,
    pub cc: Vec<String>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            context: super::context_field(),
            ..Self::default()
        }
    }
}
