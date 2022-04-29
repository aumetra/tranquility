// Warnings related to those lints are caused by expanded SQLx code
#![allow(clippy::similar_names)]

use {
    crate::error::Error,
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    sqlx::PgPool,
    uuid::Uuid,
};

pub mod connection {
    use {sqlx::PgPool, std::future::Future};

    pub fn init_pool(db_url: &'_ str) -> impl Future<Output = Result<PgPool, sqlx::Error>> + '_ {
        PgPool::connect(db_url)
    }
}

#[async_trait]
/// Convenience extension trait. Allows insertion via an immutable reference to a database pool
pub trait InsertExt: ormx::Insert {
    /// Insert a row into the database, returning the inserted row
    async fn insert(
        self,
        conn_pool: &sqlx::Pool<ormx::Db>,
    ) -> Result<<Self as ormx::Insert>::Table, Error> {
        // Acquire a connection from the database pool
        let mut db_conn = conn_pool.acquire().await?;

        ormx::Insert::insert(self, &mut db_conn).await
    }
}

#[async_trait]
impl<T> InsertExt for T where T: ormx::Insert {}

/// Wrapper struct for the query of the [last_activity_timestamp] function
struct ObjectTimestamp {
    timestamp: DateTime<Utc>,
}

impl From<ObjectTimestamp> for DateTime<Utc> {
    fn from(timestamp: ObjectTimestamp) -> Self {
        timestamp.timestamp
    }
}

#[inline]
/// Get the timestamp of the activity. If the activity doesn't exist, default to the current time
async fn last_activity_timestamp(
    conn_pool: &PgPool,
    last_activity_id: Option<Uuid>,
) -> Result<DateTime<Utc>, Error> {
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
    // Either return the current time or convert it via the `Into` trait
    .map_or_else(|_| Utc::now(), Into::into);

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
pub mod oauth;
pub mod object;
pub mod outbox;

pub use actor::*;
pub use oauth::*;
pub use object::*;
