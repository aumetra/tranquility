use {crate::error::Error, cached::proc_macro::cached};

#[cached(
    result,
    size = 50,
    time = 15,
    key = "String",
    convert = r#"{ followed_url.to_owned() }"#
)]
pub async fn select(followed_url: &str) -> Result<Vec<String>, Error> {
    let conn_pool = crate::database::connection::get().await?;

    let owner_urls = sqlx::query_as::<_, (String,)>(
        r#"
            SELECT data->>'actor' FROM objects
            WHERE data->>'type' = 'Follow'
            AND data->>'object' = $2
        "#,
    )
    .bind(followed_url)
    .fetch_all(conn_pool)
    .await?
    .into_iter()
    .map(|(url,)| url)
    .collect();

    Ok(owner_urls)
}
