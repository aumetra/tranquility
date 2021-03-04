use {
    crate::{database::model::OAuthToken, error::Error},
    chrono::NaiveDateTime,
    uuid::Uuid,
};

pub async fn insert(
    application_id: Option<Uuid>,
    actor_id: Uuid,
    access_token: String,
    refresh_token: Option<String>,
    valid_until: NaiveDateTime,
) -> Result<OAuthToken, Error> {
    let conn_pool = crate::database::connection::get();

    let token = sqlx::query_as!(
        OAuthToken,
        r#"
            INSERT INTO oauth_tokens
            ( application_id, actor_id, access_token, refresh_token, valid_until )
            VALUES
            ( $1, $2, $3, $4, $5 )
            RETURNING *
        "#,
        application_id,
        actor_id,
        access_token,
        refresh_token,
        valid_until,
    )
    .fetch_one(conn_pool)
    .await?;

    Ok(token)
}

pub mod select {
    use crate::{database::model::OAuthToken, error::Error};

    pub async fn by_token(token: &str) -> Result<OAuthToken, Error> {
        let conn_pool = crate::database::connection::get();

        let token = sqlx::query_as!(
            OAuthToken,
            r#"
                SELECT * FROM oauth_tokens
                WHERE access_token = $1
            "#,
            token
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(token)
    }
}
