use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tag {
    pub r#type: String,
    // Format: @<username>@<instance>
    pub name: String,
    pub href: String,
}
