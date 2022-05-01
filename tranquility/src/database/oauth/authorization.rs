use crate::error::Error;
use ormx::Table;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Table)]
#[ormx(id = id, table = "oauth_authorizations", deletable, insertable)]
pub struct OAuthAuthorization {
    #[ormx(default)]
    pub id: Uuid,

    pub application_id: Uuid,
    pub actor_id: Uuid,

    #[ormx(get_one(&str))]
    pub code: String,

    pub valid_until: OffsetDateTime,

    #[ormx(default)]
    pub created_at: OffsetDateTime,

    #[ormx(default)]
    pub updated_at: OffsetDateTime,
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
