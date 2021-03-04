// Warnings related to those lints are caused by expanded SQLx code
#![allow(clippy::used_underscore_binding, clippy::similar_names)]

use {
    crate::error::Error,
    chrono::{NaiveDateTime, Utc},
    uuid::Uuid,
};

pub mod connection {
    use {once_cell::sync::OnceCell, sqlx::postgres::PgPool};

    static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

    pub fn get() -> &'static PgPool {
        DATABASE_POOL.get().unwrap()
    }

    pub async fn init_pool(db_url: &str) {
        let conn_pool = PgPool::connect(db_url)
            .await
            .expect("Couldn't initialize connection to database");

        DATABASE_POOL.set(conn_pool).ok();
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
    let conn = crate::database::connection::get();

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

pub async fn migrate() -> Result<(), Error> {
    let conn_pool = connection::get();
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
