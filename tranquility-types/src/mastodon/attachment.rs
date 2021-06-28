use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing an [Mastodon attachment](https://docs.joinmastodon.org/entities/attachment/)
pub struct Attachment {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub preview_url: String,
    pub remote_url: Option<String>,
    pub text_url: String,
    pub meta: Option<Meta>,
    pub description: String,
    pub blurhash: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the meta field of an [Attachment] struct
pub struct Meta {
    pub original: MetaSize,
    pub small: MetaSize,
    pub focus: MetaFocus,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the different sizes of a [Meta] struct
pub struct MetaSize {
    pub width: i64,
    pub height: i64,
    pub size: String,
    pub aspect: f64,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
/// Struct representing the [focal points](https://docs.joinmastodon.org/methods/statuses/media/#focal-points)
pub struct MetaFocus {
    pub x: f64,
    pub y: f64,
}
