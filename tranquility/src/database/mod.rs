// Warnings related to those lints are caused by expanded SQLx code
#![allow(clippy::used_underscore_binding, clippy::similar_names)]

use {
    crate::error::Error,
    chrono::{NaiveDateTime, Utc},
    uuid::Uuid,
};

pub mod connection {
    use {crate::error::Error, once_cell::sync::OnceCell, sqlx::postgres::PgPool};

    static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

    pub async fn get() -> Result<&'static PgPool, Error> {
        let value = if let Some(val) = DATABASE_POOL.get() {
            val
        } else {
            let config = crate::config::get();

            let conn_pool = PgPool::connect(&config.server.database_url).await?;
            DATABASE_POOL.set(conn_pool).unwrap();

            DATABASE_POOL.get().unwrap()
        };

        Ok(value)
    }
}

struct ObjectTimestamp {
    timestamp: NaiveDateTime,
}

impl Into<NaiveDateTime> for ObjectTimestamp {
    fn into(self) -> NaiveDateTime {
        self.timestamp
    }
}

#[inline]
async fn last_activity_timestamp(last_activity_id: Option<Uuid>) -> Result<NaiveDateTime, Error> {
    let conn = crate::database::connection::get().await?;

    let last_timestamp = sqlx::query_as!(
        ObjectTimestamp,
        r#"
            SELECT created_at as "timestamp!" FROM objects
            WHERE id = $1
        "#,
        last_activity_id,
    )
    .fetch_one(conn)
    .await
    .map_or_else(|_| Utc::now().naive_local(), Into::into);

    Ok(last_timestamp)
}

pub async fn init() -> Result<(), Error> {
    let conn_pool = connection::get().await?;
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
