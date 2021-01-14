use {
    crate::{cpu_intensive_work, database::model::Actor as DBActor, error::Error},
    futures_util::stream::{FuturesUnordered, StreamExt},
    itertools::Itertools,
    reqwest::{
        header::{HeaderName, HeaderValue, DATE},
        Client, Request, Response,
    },
    std::{future::Future, sync::Arc},
    tranquility_types::activitypub::{Activity, Actor, PUBLIC_IDENTIFIER},
};

async fn prepare_request(
    client: &Client,
    url: &str,
    author: Arc<Actor>,
    author_db: Arc<DBActor>,
    activity: &Activity,
) -> Result<Request, Error> {
    let mut request = client
        .post(url)
        .header("Content-Type", "application/activity+json")
        .json(activity)
        .build()?;

    let date_header_value = HeaderValue::from_str(&chrono::Utc::now().to_rfc2822())?;

    let activity_bytes = serde_json::to_vec(&activity)?;
    let digest_header_value = crate::crypto::digest::http_header(activity_bytes).await?;

    request.headers_mut().insert(DATE, date_header_value);
    request
        .headers_mut()
        .insert(HeaderName::from_static("digest"), digest_header_value);

    let (header_name, header_value) = {
        let request = request.try_clone().unwrap();

        cpu_intensive_work!(move || {
            let key_id = author.public_key.id.as_str();
            let private_key = author_db.private_key.as_ref().unwrap();

            http_signatures::sign(
                &request,
                key_id,
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

fn create_delivery_futures<'a>(
    activity: &Arc<Activity>,
    author: &Arc<Actor>,
    author_db: &Arc<DBActor>,
    recipient_list: Vec<&'a str>,
) -> FuturesUnordered<impl Future<Output = Result<Response, Error>> + 'a> {
    let delivery_futures = FuturesUnordered::new();

    for url in recipient_list {
        // Create a new atomic reference for the activity data
        let activity = Arc::clone(activity);

        // Create new atomic references for the author data
        let author = Arc::clone(author);
        let author_db = Arc::clone(author_db);

        let outgoing_request = async move {
            debug!("Delivering activity {} to actor {}...", activity.id, url);

            let client = &crate::REQWEST_CLIENT;
            let request = prepare_request(client, url, author, author_db, &activity).await?;

            client.execute(request).await.map_err(Error::from)
        };

        delivery_futures.push(outgoing_request);
    }

    delivery_futures
}

async fn resolve_delivery_futures<F>(mut futures: FuturesUnordered<F>)
where
    F: Future<Output = Result<Response, Error>>,
{
    while let Some(delivery_result) = futures.next().await {
        match delivery_result {
            Ok(response) if response.status().is_success() => (),
            Ok(response) => {
                let response_status = response.status();
                let response_body = response.text().await.unwrap_or_default();

                warn!(
                    "Delivery request wasn't successful\nStatus code: {}\nServer response: {}",
                    response_status, response_body,
                )
            }
            Err(err) => warn!("Delivery request failed: {}", err),
        }
    }
}

pub async fn deliver(activity: Activity) -> Result<(), Error> {
    let activity = Arc::new(activity);

    let (author, author_db) =
        crate::activitypub::fetcher::fetch_actor(activity.actor.as_str()).await?;
    let author = Arc::new(author);
    let author_db = Arc::new(author_db);

    tokio::spawn(async move {
        // TODO: Resolve follow collections
        // TODO: Resolve the recipient list (it contains the URL to their actors but not to their inboxes)
        let recipient_list = activity
            .to
            .iter()
            .merge(activity.cc.iter())
            .unique()
            .filter_map(|url| {
                if *url == PUBLIC_IDENTIFIER {
                    None
                } else {
                    Some(url.as_str())
                }
            })
            .collect_vec();

        let delivery_futures =
            create_delivery_futures(&activity, &author, &author_db, recipient_list);
        resolve_delivery_futures(delivery_futures).await;
    });

    Ok(())
}
