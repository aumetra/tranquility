use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing a [Mastodon emoji](https://docs.joinmastodon.org/entities/emoji/)
pub struct Emoji {
    pub shortcode: String,

    pub url: String,
    pub static_url: String,

    pub visible_in_picker: bool,
}
