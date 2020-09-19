use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Activity {
    #[serde(rename = "@context")]
    pub _context: Value,

    pub id: String,
    pub r#type: String,
    // Link to the actor this activity belongs to
    pub actor: String,

    // Value type because this can either be a generic object or an actor
    pub object: Value,

    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
}
