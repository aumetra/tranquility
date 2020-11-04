use {chrono::NaiveDateTime, serde_json::Value, uuid::Uuid};

#[derive(Clone)]
pub struct Actor {
    pub id: Uuid,

    pub username: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub private_key: Option<String>,

    pub actor: Value,
    pub remote: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct Object {
    pub id: Uuid,

    pub owner_id: Uuid,

    pub data: Value,
    pub url: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct OAuthApplication {
    pub id: Uuid,

    pub client_name: String,
    pub client_id: Uuid,
    pub client_secret: String,

    pub redirect_uris: String,
    pub scopes: String,
    pub website: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct OAuthAuthorization {
    pub id: Uuid,

    pub application_id: Uuid,
    pub actor_id: Uuid,

    pub code: String,
    pub valid_until: NaiveDateTime,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct OAuthToken {
    pub id: Uuid,

    pub application_id: Uuid,
    pub actor_id: Uuid,

    pub access_token: String,
    pub refresh_token: Option<String>,
    pub valid_until: NaiveDateTime,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
