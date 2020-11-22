use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Emoji {
    pub shortcode: String,

    pub url: String,
    pub static_url: String,

    pub visible_in_picker: bool,
}
