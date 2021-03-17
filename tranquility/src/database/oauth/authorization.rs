use {
    crate::{database::model::OAuthAuthorization, error::Error},
    chrono::NaiveDateTime,
    sqlx::PgPool,
    uuid::Uuid,
};

pub mod delete {
    use {crate::error::Error, sqlx::PgPool};

    pub async fn expired(conn_pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            .await?;

        Ok(())
    }
}

pub mod select {
    use {
        crate::{database::model::OAuthAuthorization, error::Error},
        sqlx::PgPool,
    };

    pub async fn by_code(conn_pool: &PgPool, code: &str) -> Result<OAuthAuthorization, Error> {
        let authorization = sqlx::query_as!(
            OAuthAuthorization,
            r#"
                SELECT * FROM oauth_authorizations
                WHERE code = $1
            "#,
            code
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(authorization)
    }
}

pub async fn insert(
    conn_pool: &PgPool,
    application_id: Uuid,
    actor_id: Uuid,
    code: String,
    valid_until: NaiveDateTime,
) -> Result<OAuthAuthorization, Error> {
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
