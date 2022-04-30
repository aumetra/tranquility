use chrono::{DateTime, Utc};
use ormx::Table;
use uuid::Uuid;

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_tokens", deletable, insertable)]
pub struct OAuthToken {
    #[ormx(default)]
    pub id: Uuid,

    pub application_id: Option<Uuid>,
    pub actor_id: Uuid,
    #[ormx(get_one(&str))]
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub valid_until: DateTime<Utc>,

    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default)]
    pub updated_at: DateTime<Utc>,
}
