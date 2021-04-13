pub mod delete {
    use {crate::error::Error, sqlx::PgPool};

    pub async fn expired(conn_pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("DELETE FROM oauth_authorizations WHERE valid_until < NOW()")
            .execute(conn_pool)
            .await?;

        Ok(())
    }
}
