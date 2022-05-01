use ormx::Table;
use time::OffsetDateTime;
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
    pub valid_until: OffsetDateTime,

    #[ormx(default)]
    pub created_at: OffsetDateTime,

    #[ormx(default)]
    pub updated_at: OffsetDateTime,
}
