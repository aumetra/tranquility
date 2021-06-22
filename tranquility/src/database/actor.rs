use {
    crate::error::Error,
    chrono::{DateTime, Utc},
    ormx::Table,
    serde_json::Value,
    sqlx::PgPool,
    uuid::Uuid,
};

#[derive(Clone, Table)]
#[ormx(id = id, table = "actors", insertable)]
pub struct Actor {
    pub id: Uuid,

    pub username: String,
    #[ormx(get_optional(&str))]
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub private_key: Option<String>,

    pub is_confirmed: bool,
    #[ormx(get_one(&str))]
    pub confirmation_code: Option<String>,

    pub actor: Value,
    pub remote: bool,

    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default)]
    pub updated_at: DateTime<Utc>,
}

impl Actor {
    /// Get an actor by their URL
    pub async fn by_url(conn_pool: &PgPool, url: &str) -> Result<Self, Error> {
        let actor = sqlx::query_as!(
            Actor,
            r#"
                SELECT * FROM actors
                WHERE actor->>'id' = $1
            "#,
            url
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(actor)
    }

    /// Get an confirmed local actor by their username
    pub async fn by_username_local(conn_pool: &PgPool, username: &str) -> Result<Self, Error> {
        let actor = sqlx::query_as!(
            Actor,
            r#"
                SELECT * FROM actors
                WHERE username = $1
                AND remote = FALSE
                AND is_confirmed = TRUE
            "#,
            username
        )
        .fetch_one(conn_pool)
        .await?;

        Ok(actor)
    }
}
