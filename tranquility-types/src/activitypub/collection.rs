use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(default = "super::context_field", rename = "@context")]
    context: Value,

    id: String,
    r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    part_of: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    prev: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    next: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ordered_items: Vec<super::Activity>,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            context: super::context_field(),
            ..Self::default()
        }
    }
}
