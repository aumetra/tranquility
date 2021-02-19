use {
    crate::error::Error,
    cached::proc_macro::cached,
    futures_util::stream::{StreamExt, TryStreamExt},
};

// Required because of the "query_as" macro
// Otherwise we couldn't use compile-time verified SQL queries
struct InboxUrl {
    inbox_url: String,
}

impl Into<String> for InboxUrl {
    fn into(self) -> String {
        self.inbox_url
    }
}

#[cached(
    result,
    size = 50,
    time = 15,
    key = "String",
    convert = r#"{ followed_url.to_owned() }"#
)]
pub async fn resolve_followers(followed_url: &str) -> Result<Vec<String>, Error> {
    let conn_pool = crate::database::connection::get().await?;

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

#[cached(
    result,
    size = 50,
    time = 15,
    key = "String",
    convert = r#"{ url.to_owned() }"#
)]
pub async fn resolve_one(url: &str) -> Result<String, Error> {
    let conn_pool = crate::database::connection::get().await?;

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
