use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Mention {
    pub id: String,

    pub username: String,
    pub acct: String,

    pub url: String,
}
