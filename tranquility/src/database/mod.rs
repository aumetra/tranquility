// Warnings related to those lints are  
// caused by expanded SQLx code
#![allow(
    clippy::toplevel_ref_arg,
    clippy::used_underscore_binding,
    clippy::similar_names
)]

use crate::error::Error;

pub mod connection {
    use {crate::error::Error, once_cell::sync::OnceCell, sqlx::postgres::PgPool};

    static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

    pub fn get() -> Result<&'static PgPool, Error> {
        if let Some(val) = DATABASE_POOL.get() {
            Ok(val)
        } else {
            let config = crate::config::get();
            let conn_pool = PgPool::connect_lazy(&config.database_url)?;
            DATABASE_POOL.set(conn_pool).unwrap();

            get()
        }
    }
}

pub async fn init() -> Result<(), Error> {
    let conn_pool = connection::get()?;
    sqlx::migrate!("../migrations").run(conn_pool).await?;

    Ok(())
}

pub mod activity;
pub mod actor;
pub mod model;
