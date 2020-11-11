use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: Option<String>,
}
