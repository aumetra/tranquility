use {
    crate::{database::model::OAuthAuthorization, error::Error},
    chrono::NaiveDateTime,
    uuid::Uuid,
};

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
