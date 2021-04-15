use {chrono::NaiveDateTime, ormx::Table, uuid::Uuid};

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_tokens", insertable)]
pub struct OAuthToken {
    #[ormx(default)]
    pub id: Uuid,

    pub application_id: Option<Uuid>,
    pub actor_id: Uuid,
    #[ormx(get_one(&str))]
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub valid_until: NaiveDateTime,

    #[ormx(default)]
    pub created_at: NaiveDateTime,
    #[ormx(default)]
    pub updated_at: NaiveDateTime,
}
