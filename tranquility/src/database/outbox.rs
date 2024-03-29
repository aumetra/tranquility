use crate::{
    database::{last_activity_timestamp, Object},
    error::Error,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Get activities for displaying on the outbox
pub async fn activities(
    conn_pool: &PgPool,
    user_id: Uuid,
    last_activity_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<Object>, Error> {
    let last_activity_timestamp = last_activity_timestamp(conn_pool, last_activity_id).await?;
    let create_activities = sqlx::query_as!(
        Object,
        r#"
            SELECT * FROM objects
            WHERE owner_id = $1
            AND data->>'type' = 'Create'
            AND created_at < $2
            LIMIT $3
        "#,
        user_id,
        last_activity_timestamp,
        limit
    )
    .fetch_all(conn_pool)
    .await?;

    Ok(create_activities)
}
