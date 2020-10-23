use {crate::error::Error, tranquility_types::activitypub::Actor};

pub async fn update(actor: Actor) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get()?;

    let username = actor.username.clone();
    let url = actor.id.clone();
    let actor = serde_json::to_value(actor)?;

    sqlx::query!(
        r#"
            UPDATE actors
            SET actor = $1, username = $2
            WHERE actor->>'id' = $3
        "#,
        actor,
        username,
        url
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}

pub mod insert {
    use {
        crate::error::Error, serde_json::Value, tranquility_types::activitypub::Actor, uuid::Uuid,
    };

    pub async fn local(
        id: Uuid,
        actor: Actor,
        email: String,
        password: String,
        private_key_pem: String,
    ) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        let actor_value = serde_json::to_value(&actor)?;
        sqlx::query!(
            r#"
                INSERT INTO actors
                ( id, username, email, password_hash, private_key, actor ) 
                VALUES 
                ( $1, $2, $3, $4, $5, $6 )
            "#,
            id,
            actor.username,
            email,
            password,
            private_key_pem,
            actor_value
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }

    pub async fn remote(username: String, actor: Value) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        sqlx::query!(
            r#"
                INSERT INTO actors
                ( username, actor, remote )
                VALUES 
                ( $1, $2, TRUE )
            "#,
            username,
            actor
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }
}

pub mod select {
    use {
        crate::{database::model::Actor, error::Error},
        uuid::Uuid,
    };

    pub async fn by_id(id: Uuid) -> Result<Actor, Error> {
        let conn_pool = crate::database::connection::get()?;

        let actor = sqlx::query_as!(
            Actor,
            r#"
                SELECT * FROM actors
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(actor)
    }

    pub async fn by_url(url: String) -> Result<Actor, Error> {
        let conn_pool = crate::database::connection::get()?;

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
}
