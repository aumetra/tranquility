pub mod insert {
    use {
        crate::{database::model::Actor, error::Error},
        serde_json::Value,
        tranquility_types::activitypub::actor,
    };

    async fn is_username_taken(username: &String) -> Result<bool, Error> {
        let conn_pool = crate::database::connection::get()?;

        let num_rows = sqlx::query!("SELECT COUNT(*) FROM actors WHERE username = $1", username)
            .fetch_one(conn_pool)
            .await
            .map(|result| result.count.unwrap_or(0))?;

        Ok(num_rows > 0)
    }

    pub async fn local(username: String, email: String, password: String) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        if is_username_taken(&username).await? {
            return Err(Error::DuplicateUsername);
        }

        // Hash the password and generate the RSA key pair
        let password_hash = crate::crypto::password::hash(password).await?;

        let rsa_private_key = crate::crypto::rsa::generate().await?;
        let (public_key_pem, private_key_pem) = crate::crypto::rsa::to_pem(rsa_private_key)?;

        let mut transaction = conn_pool.begin().await?;

        let actor = sqlx::query_as!(
            Actor,
            r#"
                INSERT INTO actors
                ( username, email, password_hash, private_key, actor ) 
                VALUES 
                ( $1, $2, $3, $4, $5 )
                RETURNING *
                "#,
            username,
            email,
            password_hash,
            private_key_pem,
            Value::default()
        )
        .fetch_one(&mut transaction)
        .await?;

        let config = crate::config::get();
        let ap_actor = actor::create(
            &actor.id.to_simple_ref().to_string(),
            &actor.username,
            public_key_pem,
            &config.domain,
        );
        let ap_actor = serde_json::to_value(ap_actor)?;

        sqlx::query!(
            r#"
                UPDATE actors
                SET actor = $1
                WHERE id = $2
                "#,
            ap_actor,
            actor.id
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn external(username: String, actor: Value) -> Result<(), Error> {
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
