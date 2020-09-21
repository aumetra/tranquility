use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

pub struct Actor {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub private_key: Option<String>,

    pub actor: Value,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Activity {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub data: Value,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
