use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Struct representing an [ActivityStreams collection](https://www.w3.org/TR/activitystreams-core/#collections)
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
    pub next: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ordered_items: Vec<Item>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Item {
    Activity(super::Activity),
    Url(String),
}

impl From<super::Activity> for Item {
    fn from(item: super::Activity) -> Self {
        Self::Activity(item)
    }
}

impl From<String> for Item {
    fn from(item: String) -> Self {
        Self::Url(item)
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            context: super::context_field(),

            id: String::default(),
            r#type: String::default(),

            first: None,

            part_of: String::default(),
            next: String::default(),

            ordered_items: Vec::default(),
        }
    }
}
