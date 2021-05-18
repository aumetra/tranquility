use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
/// Struct representing a [Mastodon field](https://docs.joinmastodon.org/entities/field/)
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: Option<String>,
}
