use {
    crate::error::Error,
    futures_util::stream::{FuturesUnordered, StreamExt},
    itertools::Itertools,
    reqwest::{
        header::{HeaderName, HeaderValue, DATE},
        Client, Request,
    },
    serde_json::Value,
    std::sync::Arc,
    tranquility_types::activitypub::{Activity, PUBLIC_IDENTIFIER},
};

async fn prepare_request(client: &Client, url: &str, activity: &Value) -> Result<Request, Error> {
    let mut request = client
        .post(url)
        .header("Content-Type", "application/activity+json")
        .json(activity)
        .build()?;

    let (remote_actor, remote_actor_db) = super::fetcher::fetch_actor(&url).await?;

    let date_header_value = HeaderValue::from_str(&chrono::Utc::now().to_rfc2822())?;

    let activity_bytes = serde_json::to_vec(&activity)?;
    let digest_header_value = crate::crypto::digest::http_header(activity_bytes).await?;

    request.headers_mut().insert(DATE, date_header_value);
    request
        .headers_mut()
        .insert(HeaderName::from_static("digest"), digest_header_value);

    let key_id = remote_actor.public_key.id;
    let private_key = remote_actor_db.private_key.unwrap();

    let (header_name, header_value) = {
        let request = request.try_clone().unwrap();

        tokio::task::spawn_blocking(move || {
            http_signatures::sign(
                &request,
                &key_id,
                &["(request-target)", "date", "digest"],
                private_key.as_bytes(),
            )
        })
        .await
        .unwrap()?
    };

    request.headers_mut().insert(header_name, header_value);

    Ok(request)
}

pub fn deliver(activity: Activity) -> Result<(), Error> {
    let activity_value = Arc::new(serde_json::to_value(&activity)?);

    let activity_id = Arc::new(activity.id);
    let recipient_list = activity
        .to
        .into_iter()
        .merge(activity.cc)
        .unique()
        .filter(|url| url != PUBLIC_IDENTIFIER)
        .collect_vec();

    tokio::spawn(async move {
        let mut delivery_futures = FuturesUnordered::new();

        for url in recipient_list {
            let activity_id = Arc::clone(&activity_id);
            let activity_value = Arc::clone(&activity_value);

            let join_handle = tokio::spawn(async move {
                debug!("Delivering activity {} to actor {}...", activity_id, url);

                let client = &crate::REQWEST_CLIENT;
                let request = prepare_request(client, url.as_str(), &activity_value).await?;

                client.execute(request).await.map_err(Error::from)
            });

            delivery_futures.push(join_handle);
        }

        while let Some(delivery_result) = delivery_futures.next().await {
            match delivery_result.unwrap() {
                Ok(_) => (),
                Err(err) => warn!("Couldn't deliver activity: {}", err),
            }
        }
    });

    Ok(())
}
