use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Tag {
    pub r#type: String,
    // Format: @<username>@<instance>
    pub name: String,
    pub href: String,
}
