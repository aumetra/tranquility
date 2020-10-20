use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    pub r#type: String,
    // Format: @<username>@<instance>
    pub name: String,
    pub href: String,
}
