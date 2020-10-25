use {
    crate::error::Error, serde_json::Value, tranquility_types::activitypub::Activity, uuid::Uuid,
};

pub async fn insert(owner_id: Uuid, activity: &Activity) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get()?;

    let url = activity.id.as_str();
    let activity = serde_json::to_value(activity)?;

    sqlx::query!(
        r#"
            INSERT INTO activities 
            ( owner_id, data, url ) 
            VALUES 
            ( $1, $2, $3 )
        "#,
        owner_id,
        activity,
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
                DELETE FROM activities
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
                DELETE FROM activities
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
                DELETE FROM activities
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
        crate::{database::model::Activity, error::Error},
        uuid::Uuid,
    };

    pub async fn by_id(id: Uuid) -> Result<Activity, Error> {
        let conn_pool = crate::database::connection::get()?;

        let activity = sqlx::query_as!(
            Activity,
            r#"
                SELECT * FROM activities
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(activity)
    }

    pub async fn by_url(url: &str) -> Result<Activity, Error> {
        let conn_pool = crate::database::connection::get()?;

        let activity = sqlx::query_as!(
            Activity,
            r#"
                SELECT * FROM activities
                WHERE url = $1
            "#,
            url
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(activity)
    }
}

pub async fn update(id: Uuid, activity: Value) -> Result<(), Error> {
    let conn_pool = crate::database::connection::get()?;

    sqlx::query!(
        r#"
            UPDATE activities
            SET data = $1
            WHERE id = $2
        "#,
        activity,
        id,
    )
    .execute(conn_pool)
    .await?;

    Ok(())
}
