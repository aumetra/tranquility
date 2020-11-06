use {
    crate::{database::model::OAuthAuthorization, error::Error},
    chrono::NaiveDateTime,
    uuid::Uuid,
};

pub mod delete {
    use crate::error::Error;

    pub async fn expired() -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            .await?;

        Ok(())
    }
}

pub async fn insert(
    application_id: Uuid,
    actor_id: Uuid,
    code: String,
    valid_until: NaiveDateTime,
) -> Result<OAuthAuthorization, Error> {
    let conn_pool = crate::database::connection::get()?;

    let authorization = sqlx::query_as!(
        OAuthAuthorization,
        r#"
            INSERT INTO oauth_authorizations
            ( application_id, actor_id, code, valid_until )
            VALUES
            ( $1, $2, $3, $4 )
            RETURNING *
        "#,
        application_id,
        actor_id,
        code,
        valid_until
    )
    .fetch_one(conn_pool)
    .await?;

    Ok(authorization)
}
