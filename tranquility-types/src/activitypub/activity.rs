use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Activity {
    #[serde(rename = "@context")]
    pub _context: Value,

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

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectField {
    Object(super::Object),
    Actor(super::Actor),
    Url(String),
}

impl Default for ObjectField {
    fn default() -> ObjectField {
        ObjectField::Object(Default::default())
    }
}

impl ObjectField {
    pub fn as_actor(&self) -> Option<&super::Actor> {
        match self {
            ObjectField::Actor(actor) => Some(actor),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&super::Object> {
        match self {
            ObjectField::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_url(&self) -> Option<&String> {
        match self {
            ObjectField::Url(url) => Some(url),
            _ => None,
        }
    }
}

impl From<super::Actor> for ObjectField {
    fn from(actor: super::Actor) -> ObjectField {
        ObjectField::Actor(actor)
    }
}

impl From<super::Object> for ObjectField {
    fn from(object: super::Object) -> ObjectField {
        ObjectField::Object(object)
    }
}

impl From<String> for ObjectField {
    fn from(url: String) -> ObjectField {
        ObjectField::Url(url)
    }
}
