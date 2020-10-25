use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize)]
pub struct Activity {
    #[serde(default = "super::context_field", rename = "@context")]
    pub context: Value,

    pub id: String,
    pub r#type: String,
    // Link to the actor this activity belongs to
    pub actor: String,

    // This can either be an "Actor", "Object" or an URL to either of those
    pub object: ObjectField,

    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            context: super::context_field(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectField {
    Object(super::Object),
    Actor(super::Actor),
    Url(String),
}

impl Default for ObjectField {
    fn default() -> Self {
        Self::Object(Default::default())
    }
}

impl ObjectField {
    pub fn as_actor(&self) -> Option<&super::Actor> {
        match self {
            Self::Actor(actor) => Some(actor),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&super::Object> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_url(&self) -> Option<&String> {
        match self {
            Self::Url(url) => Some(url),
            _ => None,
        }
    }
}

impl From<super::Actor> for ObjectField {
    fn from(actor: super::Actor) -> Self {
        Self::Actor(actor)
    }
}

impl From<super::Object> for ObjectField {
    fn from(object: super::Object) -> Self {
        Self::Object(object)
    }
}

impl From<String> for ObjectField {
    fn from(url: String) -> Self {
        Self::Url(url)
    }
}
