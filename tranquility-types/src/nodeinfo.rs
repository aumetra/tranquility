use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Struct representing an entry in the "links" array
pub struct Link {
    pub rel: String,
    pub href: String,
}

impl Link {
    /// Initialise a new nodeinfo link
    ///
    /// The "rel" of this value will point to "http://nodeinfo.diaspora.software/ns/schema/2.1" because the only types available here are 2.1 types
    pub fn new(href: String) -> Self {
        Self {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".into(),
            href,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Struct representing a collection of links pointing to Nodeinfo entities
pub struct LinkCollection {
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Struct representing a [Nodeinfo 2.1](https://github.com/jhass/nodeinfo/blob/1fcd229a84031253eb73a315e89d3f7f13f117b4/PROTOCOL.md) entity
pub struct Nodeinfo {
    pub version: String,
    pub software: Software,
    pub protocols: Vec<String>,
    pub services: Services,
    pub open_registrations: bool,
    pub usage: Usage,
    pub metadata: Value,
}

impl Default for Nodeinfo {
    fn default() -> Self {
        Self {
            version: "2.1".into(),
            software: Software::default(),
            protocols: Vec::new(),
            services: Services::default(),
            open_registrations: false,
            usage: Usage::default(),

            // Has to be an empty map to comply with the schema
            metadata: Value::Object(Map::default()),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Struct representing the `usage` field of a Nodeinfo entity
pub struct Usage {
    pub users: UsageUsers,
    pub local_posts: u64,
    pub local_comments: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
/// Struct representing the `users` field of a "Usage" entity
pub struct UsageUsers {
    pub total: u64,
    pub active_halfyear: u64,
    pub active_month: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Struct representing the `software` field of a Nodeinfo entity
pub struct Software {
    pub name: String,
    pub version: String,
    pub repository: String,
    pub homepage: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Struct representing the `services` field of a Nodeinfo entity
pub struct Services {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}
