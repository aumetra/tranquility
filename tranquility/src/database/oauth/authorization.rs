use {
    crate::{database::model::OAuthAuthorization, error::Error},
    chrono::NaiveDateTime,
    tokio_compat_02::FutureExt,
    uuid::Uuid,
};

pub mod delete {
    use {crate::error::Error, tokio_compat_02::FutureExt};

    pub async fn expired() -> Result<(), Error> {
        let conn_pool = crate::database::connection::get().await?;

        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            // SQLx isn't on Tokio 1.0 yet
            .compat()
            .await?;

        Ok(())
    }
}

pub mod select {
    use {
        crate::{database::model::OAuthAuthorization, error::Error},
        tokio_compat_02::FutureExt,
    };

    pub async fn by_code(code: &str) -> Result<OAuthAuthorization, Error> {
        let conn_pool = crate::database::connection::get().await?;

        let authorization = sqlx::query_as!(
            OAuthAuthorization,
            r#"
                SELECT * FROM oauth_authorizations
                WHERE code = $1
            "#,
            code
        )
        .fetch_one(conn_pool)
        // SQLx isn't on Tokio 1.0 yet
        .compat()
        .await?;

        Ok(authorization)
    }
}

pub async fn insert(
    application_id: Uuid,
    actor_id: Uuid,
    code: String,
    valid_until: NaiveDateTime,
) -> Result<OAuthAuthorization, Error> {
    let conn_pool = crate::database::connection::get().await?;

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
    // SQLx isn't on Tokio 1.0 yet
    .compat()
    .await?;

    Ok(authorization)
}
