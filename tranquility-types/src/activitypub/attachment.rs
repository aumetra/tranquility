use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    pub r#type: String,
    pub url: String,
}
