use crate::error::Error;

pub mod connection {
    use {crate::error::Error, once_cell::sync::OnceCell, sqlx::postgres::PgPool};

    static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();

    pub fn get() -> Result<&'static PgPool, Error> {
        match DATABASE_POOL.get() {
            Some(val) => Ok(val),
            None => {
                let config = crate::config::get();
                let conn_pool = PgPool::connect_lazy(&config.database_url)?;
                DATABASE_POOL.set(conn_pool).unwrap();

                get()
            }
        }
    }
}

pub async fn init() -> Result<(), Error> {
    let conn_pool = connection::get()?;
    sqlx::migrate!("../migrations").run(conn_pool).await?;

    Ok(())
}

pub mod actor;
pub mod model;
