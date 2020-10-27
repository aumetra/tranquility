use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Attachment {
    pub r#type: String,
    pub url: String,
}
