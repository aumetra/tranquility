use crate::error::Error;
use futures_util::stream::{StreamExt, TryStreamExt};
use sqlx::PgPool;

// Required because of the "query_as" macro
// Otherwise we couldn't use compile-time verified SQL queries
struct InboxUrl {
    inbox_url: String,
}

impl From<InboxUrl> for String {
    fn from(inbox_url: InboxUrl) -> Self {
        inbox_url.inbox_url
    }
}

/// Get the inbox URLs of the actors who are following the actor
pub async fn resolve_followers(
    conn_pool: &PgPool,
    followed_url: &str,
) -> Result<Vec<String>, Error> {
    let inbox_urls = sqlx::query_as!(
        InboxUrl,
        r#"
            SELECT actors.actor->>'inbox' as "inbox_url!" 
            FROM actors, objects
            WHERE objects.data->>'type' = 'Follow'
            AND objects.data->>'object' = $1
            AND objects.data->>'object' = actors.actor->>'id'
        "#,
        followed_url
    )
    .fetch(conn_pool)
    .map(|row_result| row_result.map(Into::into))
    .try_collect()
    .await?;

    Ok(inbox_urls)
}

/// Get the inbox URL of an actor
pub async fn resolve_one(conn_pool: &PgPool, url: &str) -> Result<String, Error> {
    let inbox_url = sqlx::query_as!(
        InboxUrl,
        // The `as "inbox_url!"` is needed here for the `query_as` macro to be
        // able to bind the result to the `inbox_url` field of the `InboxUrl` struct
        r#"
            SELECT actor->>'inbox' as "inbox_url!" 
            FROM actors
            WHERE actor->>'id' = $1
        "#,
        url,
    )
    .fetch_one(conn_pool)
    .await?;

    Ok(inbox_url.into())
}
