use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(default = "super::context_field", rename = "@context")]
    pub context: Value,

    pub id: String,
    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub part_of: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub prev: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub next: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ordered_items: Vec<super::Activity>,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            context: super::context_field(),

            id: String::default(),
            r#type: String::default(),

            first: None,

            part_of: String::default(),
            prev: String::default(),
            next: String::default(),

            ordered_items: Vec::default(),
        }
    }
}
