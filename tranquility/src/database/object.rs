use {crate::error::Error, serde_json::Value, uuid::Uuid};

pub async fn insert(owner_id: Uuid, url: &str, object: Value) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get()?;

    sqlx::query!(
        r#"
            INSERT INTO objects 
            ( owner_id, data, url ) 
            VALUES 
            ( $1, $2, $3 )
        "#,
        owner_id,
        object,
        url
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}

pub mod delete {
    use {crate::error::Error, uuid::Uuid};

    pub async fn by_id(id: Uuid) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

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
        let conn_pool = crate::database::connection::get()?;

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

    pub async fn by_object_url(url: &str) -> Result<(), Error> {
        let conn_pool = crate::database::connection::get()?;

        sqlx::query!(
            r#"
                DELETE FROM objects
                WHERE data->'object'->>'id' = $1
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
        uuid::Uuid,
    };

    pub async fn by_id(id: Uuid) -> Result<Object, Error> {
        let conn_pool = crate::database::connection::get()?;

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

    pub async fn by_type_and_owner(
        r#type: &str,
        owner_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Object>, Error> {
        let conn_pool = crate::database::connection::get()?;

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

    pub async fn by_url(url: &str) -> Result<Object, Error> {
        let conn_pool = crate::database::connection::get()?;

        let object = sqlx::query_as!(
            Object,
            r#"
                SELECT * FROM objects
                WHERE url = $1
            "#,
            url
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(object)
    }
}

pub async fn update(id: Uuid, object: Value) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get()?;

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
