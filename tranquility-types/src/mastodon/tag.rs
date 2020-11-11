use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,

    pub history: Option<History>,
}

#[derive(Deserialize, Serialize)]
pub struct History {
    pub day: String,
    pub uses: String,
    pub accounts: String,
}
