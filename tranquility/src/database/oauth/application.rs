use ormx::Table;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_applications", deletable, insertable)]
pub struct OAuthApplication {
    #[ormx(default)]
    pub id: Uuid,

    pub client_name: String,
    #[ormx(get_one)]
    pub client_id: Uuid,
    pub client_secret: String,

    pub redirect_uris: String,
    pub scopes: String,
    pub website: String,

    #[ormx(default)]
    pub created_at: OffsetDateTime,

    #[ormx(default)]
    pub updated_at: OffsetDateTime,
}
