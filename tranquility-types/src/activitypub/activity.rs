use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Struct representing an [ActivityStreams activity](https://www.w3.org/TR/activitystreams-core/#activities)
pub struct Activity {
    #[serde(default = "super::context_field", rename = "@context")]
    pub context: Value,

    pub id: String,
    pub r#type: String,
    /// Link to the actor this activity belongs to
    pub actor: String,

    /// This can either be an "Actor", "Object" or an URL to either of those
    pub object: ObjectField,
    #[serde(default)]
    pub published: String,

    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            context: super::context_field(),

            id: String::default(),
            r#type: String::default(),
            actor: String::default(),

            object: ObjectField::default(),
            published: String::default(),

            to: Vec::default(),
            cc: Vec::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectField {
    Actor(super::Actor),
    Object(super::Object),
    Url(String),
}

impl Default for ObjectField {
    fn default() -> Self {
        Self::Object(super::Object::default())
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

    pub fn as_mut_actor(&mut self) -> Option<&mut super::Actor> {
        match self {
            Self::Actor(actor) => Some(actor),
            _ => None,
        }
    }

    pub fn as_mut_object(&mut self) -> Option<&mut super::Object> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_mut_url(&mut self) -> Option<&mut String> {
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
