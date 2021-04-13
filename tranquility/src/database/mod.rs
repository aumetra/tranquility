// Warnings related to those lints are caused by expanded SQLx code
#![allow(clippy::used_underscore_binding, clippy::similar_names)]

use {
    crate::error::Error,
    chrono::{NaiveDateTime, Utc},
    sqlx::PgPool,
    uuid::Uuid,
};

pub mod connection {
    use {sqlx::PgPool, std::future::Future};

    pub fn init_pool(db_url: &'_ str) -> impl Future<Output = Result<PgPool, sqlx::Error>> + '_ {
        PgPool::connect(db_url)
    }
}

struct ObjectTimestamp {
    timestamp: NaiveDateTime,
}

impl From<ObjectTimestamp> for NaiveDateTime {
    fn from(timestamp: ObjectTimestamp) -> Self {
        timestamp.timestamp
    }
}

#[inline]
async fn last_activity_timestamp(
    conn_pool: &PgPool,
    last_activity_id: Option<Uuid>,
) -> Result<NaiveDateTime, Error> {
    let last_timestamp = sqlx::query_as!(
        ObjectTimestamp,
        r#"
            SELECT created_at as "timestamp!" FROM objects
            WHERE id = $1
        "#,
        last_activity_id,
    )
    .fetch_one(conn_pool)
    .await
    .map_or_else(|_| Utc::now().naive_local(), Into::into);

    Ok(last_timestamp)
}

/// Execute the embedded database migrations
pub async fn migrate(conn_pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("../migrations").run(conn_pool).await?;

    Ok(())
}

pub mod actor;
pub mod follow;
pub mod inbox_urls;
pub mod model;
pub mod oauth;
pub mod object;
pub mod outbox;
