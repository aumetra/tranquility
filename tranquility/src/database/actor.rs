pub mod insert {
    use crate::{database::model::Actor, error::Error};
    use futures_util::future::TryFutureExt;
    use serde_json::Value;
    use tranquility_types::activitypub::actor;

    pub async fn local(
        username: String,
        email: String,
        password_hash: String,
        public_key: String,
        private_key: String,
    ) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        // Check if the username is unique
        let num_rows = sqlx::query!("SELECT COUNT(*) FROM actors WHERE username = $1", username)
            .fetch_one(conn_pool)
            .map_ok(|row| row.count.unwrap_or_default())
            .await?;
        if num_rows > 0 {
            return Err(Error::DuplicateUsername);
        }

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
            private_key,
            Value::default()
        )
        .fetch_one(conn_pool)
        .await?;

        let config = crate::config::get();
        let ap_actor = actor::create(
            &actor.id.to_simple_ref().to_string(),
            &actor.username,
            public_key,
            &config.domain,
        );
        let ap_actor = serde_json::to_value(ap_actor)?;

        log::warn!("{}", actor.id);

        sqlx::query!(
            r#"
                UPDATE actors
                SET actor = $1
                WHERE id = $2
                "#,
            ap_actor,
            actor.id
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }

    pub async fn external(username: String, actor: Value) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        sqlx::query!(
            r#"
                INSERT INTO actors
                ( username, actor )
                VALUES 
                ( $1, $2 )
                "#,
            username,
            actor
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }
}
