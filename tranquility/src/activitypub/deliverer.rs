#![allow(clippy::needless_lifetimes)]

use {
    crate::{crypto, database::model::Actor as DbActor, error::Error},
    async_recursion::async_recursion,
    futures_util::stream::{FuturesUnordered, StreamExt},
    itertools::Itertools,
    reqwest::{
        header::{HeaderName, HeaderValue, DATE},
        Client, Request, Response,
    },
    std::{future::Future, sync::Arc},
    tranquility_types::activitypub::{Activity, Actor, PUBLIC_IDENTIFIER},
};

struct DeliveryData {
    author: Actor,
    author_db: DbActor,
    activity: Activity,
}

impl DeliveryData {
    async fn new(activity: Activity) -> Result<Arc<Self>, Error> {
        let (author, author_db) =
            crate::activitypub::fetcher::fetch_actor(activity.actor.as_str()).await?;

        let delivery_data = DeliveryData {
            author,
            author_db,
            activity,
        };

        Ok(Arc::new(delivery_data))
    }
}

async fn prepare_request(
    client: &Client,
    url: &str,
    delivery_data: Arc<DeliveryData>,
) -> Result<Request, Error> {
    let activity = &delivery_data.activity;
    let author = &delivery_data.author;
    let author_db = &delivery_data.author_db;

    let mut request = client
        .post(url)
        .header("Content-Type", "application/activity+json")
        .json(activity)
        .build()?;

    let date_header_value = HeaderValue::from_str(&chrono::Utc::now().to_rfc2822())?;

    let activity_bytes = serde_json::to_vec(activity)?;
    let digest_header_value = crate::crypto::digest::http_header(activity_bytes).await?;

    request.headers_mut().insert(DATE, date_header_value);
    request
        .headers_mut()
        .insert(HeaderName::from_static("digest"), digest_header_value);

    let (header_name, header_value) = {
        let request = request.try_clone().unwrap();
        let key_id = author.public_key.id.clone();
        let private_key = author_db.private_key.as_ref().unwrap().clone();

        crypto::request::sign(request, key_id, private_key).await?
    };

    request.headers_mut().insert(header_name, header_value);

    Ok(request)
}

fn construct_deliver_future(
    delivery_data: &Arc<DeliveryData>,
    url: String,
) -> impl Future<Output = Result<Response, Error>> + Send {
    let delivery_data = Arc::clone(delivery_data);

    async move {
        debug!(
            "Delivering activity {} to actor {}...",
            delivery_data.activity.id, url
        );

        let client = &crate::util::REQWEST_CLIENT;
        let request = prepare_request(client, url.as_str(), delivery_data).await?;

        client.execute(request).await.map_err(Error::from)
    }
}

#[async_recursion]
async fn resolve_url(delivery_data: &DeliveryData, url: String) -> Result<Vec<String>, Error> {
    // Check if the current URL is the user's follow collection
    if delivery_data.author.followers == url {
        // Get the ActivityPub IDs of all the followers
        let follower_urls =
            crate::database::inbox_urls::select(delivery_data.author.id.as_str()).await?;

        // Create futures for resolving their ID to the inbox URL
        let inbox_url_futures = follower_urls
            .into_iter()
            .map(|url| resolve_url(delivery_data, url));

        // Await all the futures one after another
        let mut inbox_urls = Vec::new();
        for inbox_url_future in inbox_url_futures {
            let urls = inbox_url_future.await?;

            inbox_urls.push(urls);
        }

        let inbox_urls = inbox_urls.into_iter().flatten().collect();

        return Ok(inbox_urls);
    }

    let (actor, _actor_db) = crate::activitypub::fetcher::fetch_actor(url.as_str()).await?;

    Ok(vec![actor.inbox])
}

async fn get_recipient_list<'a>(delivery_data: &'a DeliveryData) -> Result<Vec<String>, Error> {
    let filter_map_fn = |url: &'a String| {
        if *url == PUBLIC_IDENTIFIER {
            return None;
        }

        Some(resolve_url(delivery_data, url.to_string()))
    };

    let recipient_futures = delivery_data
        .activity
        .to
        .iter()
        .merge(delivery_data.activity.cc.iter())
        .unique()
        .filter_map(filter_map_fn)
        .collect_vec();

    let mut recipient_list = Vec::new();
    for future in recipient_futures {
        match future.await {
            Ok(url) => recipient_list.push(url),
            Err(err) => warn!("Recipient couldn't be resolved: {}", err),
        }
    }

    let recipient_list = recipient_list.into_iter().flatten().collect();

    Ok(recipient_list)
}

pub async fn deliver(activity: Activity) -> Result<(), Error> {
    let delivery_data = DeliveryData::new(activity).await?;

    tokio::spawn(async move {
        let recipient_list = match get_recipient_list(&delivery_data).await {
            Ok(list) => list,
            Err(err) => {
                warn!("Couldn't resolve recipient list: {}", err);
                return;
            }
        };

        let mut deliver_futures = recipient_list
            .into_iter()
            .map(|url| construct_deliver_future(&delivery_data, url))
            .collect::<FuturesUnordered<_>>();

        while let Some(delivery_result) = deliver_futures.next().await {
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
    });

    Ok(())
}
