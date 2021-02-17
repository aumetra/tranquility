// Warnings related to those lints are caused by expanded SQLx code
#![allow(clippy::used_underscore_binding, clippy::similar_names)]

use crate::error::Error;

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

pub async fn init() -> Result<(), Error> {
    let conn_pool = connection::get().await?;
    sqlx::migrate!("../migrations").run(conn_pool).await?;

    Ok(())
}

pub mod actor;
pub mod inbox_urls;
pub mod model;
pub mod oauth;
pub mod object;
