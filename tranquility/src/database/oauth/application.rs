use {
    crate::{database::model::OAuthApplication, error::Error},
    sqlx::PgPool,
    uuid::Uuid,
};

pub async fn insert(
    conn_pool: &PgPool,
    client_name: String,
    client_id: Uuid,
    client_secret: String,
    redirect_uris: String,
    scopes: String,
    website: String,
) -> Result<OAuthApplication, Error> {
    let client = sqlx::query_as!(
        OAuthApplication,
        r#"
            INSERT INTO oauth_applications
            ( client_name, client_id, client_secret, redirect_uris, scopes, website )
            VALUES
            ( $1, $2, $3, $4, $5, $6 )
            RETURNING *
        "#,
        client_name,
        client_id,
        client_secret,
        redirect_uris,
        scopes,
        website,
    )
    .fetch_one(conn_pool)
    .await?;

    Ok(client)
}

pub mod select {
    use {
        crate::{database::model::OAuthApplication, error::Error},
        sqlx::PgPool,
        uuid::Uuid,
    };

    pub async fn by_client_id(
        conn_pool: &PgPool,
        client_id: &Uuid,
    ) -> Result<OAuthApplication, Error> {
        let application = sqlx::query_as!(
            OAuthApplication,
            r#"
                SELECT * FROM oauth_applications
                WHERE client_id = $1
            "#,
            client_id,
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(application)
    }
}
