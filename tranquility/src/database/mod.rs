// Warnings related to those lints are caused by expanded SQLx code
#![allow(
    clippy::toplevel_ref_arg,
    clippy::used_underscore_binding,
    clippy::similar_names
)]

use {crate::error::Error, tokio_compat_02::FutureExt};

pub mod connection {
    use {
        crate::error::Error, once_cell::sync::OnceCell, sqlx::postgres::PgPool,
        tokio_compat_02::FutureExt,
    };

    static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

    pub async fn get() -> Result<&'static PgPool, Error> {
        let value = if let Some(val) = DATABASE_POOL.get() {
            val
        } else {
            let config = crate::config::get();
            // SQLx isn't on Tokio 1.0 yet
            let conn_pool = PgPool::connect(&config.database_url).compat().await?;
            DATABASE_POOL.set(conn_pool).unwrap();

            DATABASE_POOL.get().unwrap()
        };

        Ok(value)
    }
}

pub async fn init() -> Result<(), Error> {
    let conn_pool = connection::get().await?;
    sqlx::migrate!("../migrations")
        .run(conn_pool)
        // SQLx isn't on Tokio 1.0 yet
        .compat()
        .await?;

    Ok(())
}

pub mod actor;
pub mod model;
pub mod oauth;
pub mod object;
