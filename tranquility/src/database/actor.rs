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

pub mod insert {
    use {
        crate::{database::model::Actor as DbActor, error::Error},
        sqlx::PgPool,
        tranquility_types::activitypub::Actor,
        uuid::Uuid,
    };

    pub async fn local(
        conn_pool: &PgPool,
        id: Uuid,
        actor: Actor,
        email: String,
        password: String,
        private_key_pem: String,
    ) -> Result<(), Error> {
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

    pub async fn remote(
        conn_pool: &PgPool,
        username: &str,
        actor: &Actor,
    ) -> Result<DbActor, Error> {
        let actor = serde_json::to_value(actor)?;
        let db_actor = sqlx::query_as!(
            DbActor,
            r#"
                INSERT INTO actors
                ( username, actor, remote )
                VALUES 
                ( $1, $2, TRUE )
                RETURNING *
            "#,
            username,
            actor
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(db_actor)
    }
}

pub mod select {
    use {
        crate::{database::model::Actor, error::Error},
        sqlx::PgPool,
        uuid::Uuid,
    };

    pub async fn by_id(conn_pool: &PgPool, id: Uuid) -> Result<Actor, Error> {
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
