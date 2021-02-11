use {crate::error::Error, serde_json::Value, uuid::Uuid};

pub async fn insert(id: Uuid, owner_id: Uuid, object: Value) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get().await?;

    sqlx::query!(
        r#"
            INSERT INTO objects 
            ( id, owner_id, data ) 
            VALUES 
            ( $1, $2, $3 )
        "#,
        id,
        owner_id,
        object,
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}

pub mod delete {
    use {crate::error::Error, uuid::Uuid};

    pub async fn by_id(id: Uuid) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get().await?;

        sqlx::query!(
            r#"
                DELETE FROM objects
                WHERE id = $1
            "#,
            id
        )
        .execute(conn_pool)
        .await?;

        Ok(())
    }

    pub async fn by_url(url: &str) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get().await?;

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
        cached::proc_macro::cached,
        sqlx::Error as SqlxError,
        uuid::Uuid,
    };

    #[cached(result, size = 50, time = 15)]
    pub async fn by_id(id: Uuid) -> Result<Object, Error> {
        let conn_pool = crate::database::connection::get().await?;

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

    #[cached(
        result,
        size = 50,
        time = 15,
        key = "String",
        convert = r#"{ format!("{}{}{}{}", r#type, object_url, limit, offset) }"#
    )]
    pub async fn by_type_and_object_url(
        r#type: &str,
        object_url: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Object>, Error> {
        let conn_pool = crate::database::connection::get().await?;

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

    #[cached(
        result,
        size = 50,
        time = 15,
        key = "String",
        convert = r#"{ format!("{}{}{}{}", r#type, owner_id, limit, offset) }"#
    )]
    pub async fn by_type_and_owner(
        r#type: &str,
        owner_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Object>, Error> {
        let conn_pool = crate::database::connection::get().await?;

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

    #[cached(
        result,
        size = 50,
        time = 15,
        key = "String",
        convert = r#"{ format!("{}{}{}", r#type, owner_id, object_url) }"#
    )]
    pub async fn by_type_owner_and_object_url(
        r#type: &str,
        owner_id: &Uuid,
        object_url: &str,
    ) -> Result<Object, Error> {
        let conn_pool = crate::database::connection::get().await?;

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

    #[cached(
        result,
        size = 50,
        time = 15,
        key = "String",
        convert = r#"{ url.to_owned() }"#
    )]
    pub async fn by_url(url: &str) -> Result<Object, Error> {
        let conn_pool = crate::database::connection::get().await?;

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

pub async fn update(id: Uuid, object: Value) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get().await?;

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
