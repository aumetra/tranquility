use {crate::error::Error, sqlx::PgPool, tranquility_types::activitypub::Actor};

pub async fn update(conn_pool: &PgPool, actor: &Actor) -> Result<(), Error> {
    let actor_value = serde_json::to_value(actor)?;
    sqlx::query!(
        r#"
            UPDATE actors
            SET actor = $1, username = $2
            WHERE actor->>'id' = $3
        "#,
        actor_value,
        actor.username,
        actor.id
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}

pub mod select {
    use {
        crate::{database::model::Actor, error::Error},
        sqlx::PgPool,
    };

    pub async fn by_url(conn_pool: &PgPool, url: &str) -> Result<Actor, Error> {
        let actor = sqlx::query_as!(
            Actor,
            r#"
                SELECT * FROM actors
                WHERE actor->>'id' = $1
            "#,
            url
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(actor)
    }

    pub async fn by_username_local(conn_pool: &PgPool, username: &str) -> Result<Actor, Error> {
        let actor = sqlx::query_as!(
            Actor,
            r#"
                SELECT * FROM actors
                WHERE username = $1
                AND remote = FALSE
            "#,
            username
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(actor)
    }
}
