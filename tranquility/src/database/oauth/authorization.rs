use {crate::error::Error, chrono::NaiveDateTime, ormx::Table, sqlx::PgPool, uuid::Uuid};

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_authorizations", insertable)]
pub struct OAuthAuthorization {
    #[ormx(default)]
    pub id: Uuid,

    pub application_id: Uuid,
    pub actor_id: Uuid,

    #[ormx(get_one(&str))]
    pub code: String,
    pub valid_until: NaiveDateTime,

    #[ormx(default)]
    pub created_at: NaiveDateTime,
    #[ormx(default)]
    pub updated_at: NaiveDateTime,
}

impl OAuthAuthorization {
    pub async fn delete_expired(conn_pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            .await?;

        Ok(())
    }
}
