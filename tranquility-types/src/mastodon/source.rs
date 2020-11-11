use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Source {
    pub privacy: String,
    pub sensitive: bool,
    pub language: String,

    pub note: String,
    pub fields: Vec<super::Field>,
    pub follow_requests_count: i64,
}
