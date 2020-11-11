use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub original: MetaSize,
    pub small: MetaSize,
    pub focus: MetaFocus,
}

#[derive(Deserialize, Serialize)]
pub struct MetaSize {
    pub width: i64,
    pub height: i64,
    pub size: String,
    pub aspect: f64,
}

#[derive(Deserialize, Serialize)]
pub struct MetaFocus {
    pub x: f64,
    pub y: f64,
}
