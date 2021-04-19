use {
    crate::database::{last_activity_timestamp, Object},
    sqlx::PgPool,
    uuid::Uuid,
};

pub async fn followers(
    conn_pool: &PgPool,
    user_id: Uuid,
    last_activity_id: Option<Uuid>,
    limit: i64,
) -> anyhow::Result<Vec<Object>> {
    let last_activity_timestamp = last_activity_timestamp(conn_pool, last_activity_id).await?;
    let follow_activities = sqlx::query_as!(
        Object,
        r#"
            SELECT * FROM objects
            WHERE data->>'type' = 'Follow'
            AND data->>'object' = (
                SELECT actor->>'id' FROM actors
                WHERE id = $1
            )
            AND created_at < $2
            LIMIT $3
        "#,
        user_id,
        last_activity_timestamp,
        limit,
    )
    .fetch_all(conn_pool)
    .await?;

    Ok(follow_activities)
}

pub async fn following(
    conn_pool: &PgPool,
    user_id: Uuid,
    last_activity_id: Option<Uuid>,
    limit: i64,
) -> anyhow::Result<Vec<Object>> {
    let last_activity_timestamp = last_activity_timestamp(conn_pool, last_activity_id).await?;
    let follow_activities = sqlx::query_as!(
        Object,
        r#"
            SELECT * FROM objects
            WHERE data->>'type' = 'Follow'
            AND owner_id = $1
            AND created_at < $2
            LIMIT $3
        "#,
        user_id,
        last_activity_timestamp,
        limit,
    )
    .fetch_all(conn_pool)
    .await?;

    Ok(follow_activities)
}
