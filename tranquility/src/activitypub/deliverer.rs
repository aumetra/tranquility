use {
    crate::{crypto, database::Actor as DbActor, state::ArcState},
    futures_util::stream::{FuturesUnordered, StreamExt},
    itertools::Itertools,
    reqwest::{
        header::{HeaderName, HeaderValue, DATE},
        Client, Request, Response,
    },
    std::{future::Future, sync::Arc},
    tranquility_types::activitypub::{Activity, Actor, PUBLIC_IDENTIFIER},
};

/// Structure that holds data relevant to delivering an activity
struct DeliveryData {
    author: Actor,
    author_db: DbActor,
    activity: Activity,
    state: ArcState,
}

impl DeliveryData {
    async fn new(activity: Activity, state: ArcState) -> anyhow::Result<Arc<Self>> {
        let (author, author_db) =
            crate::activitypub::fetcher::fetch_actor(&state, activity.actor.as_str()).await?;

        let delivery_data = DeliveryData {
            author,
            author_db,
            activity,
            state,
        };

        Ok(Arc::new(delivery_data))
    }
}

/// Create a future that delivers the activity to the provided URL
#[instrument(
    fields(activity_id = delivery_data.activity.id.as_str()),
    skip(delivery_data),
)]
fn construct_deliver_future<'a>(
    delivery_data: &Arc<DeliveryData>,
    url: String,
) -> impl Future<Output = anyhow::Result<Response>> + Send {
    let delivery_data = Arc::clone(delivery_data);

    async move {
        debug!("Delivering activity...");

        let client = &crate::util::HTTP_CLIENT;
        let request = prepare_request(client, &url, delivery_data).await?;

        Ok(client.execute(request).await?)
    }
}

/// Prepare the HTTP request
///
/// - Sign the request
/// - Serialize the body
#[instrument(skip(client, delivery_data))]
async fn prepare_request(
    client: &Client,
    url: &str,
    delivery_data: Arc<DeliveryData>,
) -> anyhow::Result<Request> {
    let DeliveryData {
        ref activity,
        ref author,
        ref author_db,
        ..
    } = *delivery_data;

    let mut request = client
        .post(url)
        .header("Content-Type", "application/activity+json")
        .json(activity)
        .build()?;

    let date_header_value = HeaderValue::from_str(chrono::Utc::now().to_rfc2822().as_str())?;

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

/// Resolve the follow collections and the actor URL to inbox URLs
async fn resolve_url(delivery_data: &DeliveryData, url: String) -> anyhow::Result<Vec<String>> {
    // Check if the current URL is the user's follower collection
    if delivery_data.author.followers == url {
        // Get the inbox URLs of all the followers
        let inbox_urls = crate::database::inbox_urls::resolve_followers(
            &delivery_data.state.db_pool,
            delivery_data.author.id.as_str(),
        )
        .await?;

        Ok(inbox_urls)
    } else {
        // Get the inbox URL of the requested user
        let inbox_url =
            crate::database::inbox_urls::resolve_one(&delivery_data.state.db_pool, url.as_str())
                .await?;

        Ok(vec![inbox_url])
    }
}

/// De-duplicate the recipients and resolve the follow collections  
async fn get_recipient_list(delivery_data: &DeliveryData) -> anyhow::Result<Vec<String>> {
    // Merge the to and cc arrays, deduplicate them, remove the public identifier
    // and construct futures that resolve the URLs
    let recipient_futures = delivery_data
        .activity
        .to
        .iter()
        .merge(delivery_data.activity.cc.iter())
        .unique()
        .filter_map(|url| {
            (*url != PUBLIC_IDENTIFIER).then(|| resolve_url(delivery_data, url.to_string()))
        })
        .collect_vec();

    // Await all the futures one after another
    let mut recipient_list = Vec::new();
    for recipient_future in recipient_futures {
        match recipient_future.await {
            Ok(url) => recipient_list.push(url),
            Err(err) => warn!("Recipient couldn't be resolved: {}", err),
        }
    }

    // Flatten the vector of vectors of strings to a vector of strings
    let recipient_list = recipient_list.into_iter().flatten().collect();

    Ok(recipient_list)
}

/// Deliver an activity to the specified user (groups)
pub async fn deliver(activity: Activity, state: ArcState) -> anyhow::Result<()> {
    let delivery_data = DeliveryData::new(activity, state).await?;

    tokio::spawn(async move {
        let recipient_list = match get_recipient_list(&delivery_data).await {
            Ok(list) => list,
            Err(err) => {
                warn!("Couldn't resolve recipient list: {}", err);
                return;
            }
        };

        // Deliver the activity to the recipients concurrently
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
