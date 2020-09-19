use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Attachment {
    pub r#type: String,
    pub url: String,
}
