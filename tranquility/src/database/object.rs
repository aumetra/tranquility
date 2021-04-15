use {crate::error::Error, serde_json::Value, sqlx::PgPool, uuid::Uuid};

pub mod delete {
    use {crate::error::Error, sqlx::PgPool};

    pub async fn by_url(conn_pool: &PgPool, url: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
                DELETE FROM objects
                WHERE data->>'id' = $1
            "#,
            url
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }
}

pub mod select {
    use {
        crate::{database::model::Object, error::Error},
        sqlx::{Error as SqlxError, PgPool},
        uuid::Uuid,
    };

    pub async fn by_id(conn_pool: &PgPool, id: Uuid) -> Result<Object, Error> {
        let object = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(object)
    }

    pub async fn by_type_and_object_url(
        conn_pool: &PgPool,
        r#type: &str,
        object_url: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Object>, Error> {
        let objects = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE data->>'type' = $1
                AND data->>'object' = $2

                ORDER BY created_at DESC
                LIMIT $3
                OFFSET $4
            "#,
            r#type,
            object_url,
            limit,
            offset
        )
        .fetch_all(conn_pool)
        .await?;

        Ok(objects)
    }

    pub async fn by_type_and_owner(
        conn_pool: &PgPool,
        r#type: &str,
        owner_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Object>, Error> {
        let objects = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE owner_id = $1
                AND data->>'type' = $2

                ORDER BY created_at DESC
                LIMIT $3
                OFFSET $4
            "#,
            owner_id,
            r#type,
            limit,
            offset
        )
        .fetch_all(conn_pool)
        .await?;

        Ok(objects)
    }

    pub async fn by_type_owner_and_object_url(
        conn_pool: &PgPool,
        r#type: &str,
        owner_id: &Uuid,
        object_url: &str,
    ) -> Result<Object, Error> {
        let object_result = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE data->>'type' = $1
                AND owner_id = $2
                AND data->>'object' = $3

                ORDER BY created_at DESC
            "#,
            r#type,
            owner_id,
            object_url,
        )
        .fetch_one(conn_pool)
        .await;

        match object_result {
            Ok(obj) => Ok(obj),
            Err(SqlxError::RowNotFound) => Err(Error::InvalidRequest),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn by_url(conn_pool: &PgPool, url: &str) -> Result<Object, Error> {
        let object = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE data->>'id' = $1
            "#,
            url
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(object)
    }
}

pub async fn update(conn_pool: &PgPool, id: Uuid, object: Value) -> Result<(), Error> {
    sqlx::query!(
        r#"
            UPDATE objects
            SET data = $1
            WHERE id = $2
        "#,
        object,
        id,
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}
