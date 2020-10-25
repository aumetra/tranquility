use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

pub type KvPairs = HashMap<String, Option<String>>;

#[derive(Default, Deserialize, Serialize)]
pub struct Link {
    pub rel: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<KvPairs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<KvPairs>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Resource {
    pub subject: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<KvPairs>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
