use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,

    pub history: Option<History>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct History {
    pub day: String,
    pub uses: String,
    pub accounts: String,
}
