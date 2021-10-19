use {
    crate::error::Error,
    chrono::{DateTime, Utc},
    ormx::Table,
    sqlx::PgPool,
    uuid::Uuid,
};

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_authorizations", deletable, insertable)]
pub struct OAuthAuthorization {
    #[ormx(default)]
    pub id: Uuid,

    pub application_id: Uuid,
    pub actor_id: Uuid,

    #[ormx(get_one(&str))]
    pub code: String,
    pub valid_until: DateTime<Utc>,

    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default)]
    pub updated_at: DateTime<Utc>,
}

impl OAuthAuthorization {
    /// Delete all expired authorisation codes
    pub async fn delete_expired(conn_pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            .await?;

        Ok(())
    }
}
