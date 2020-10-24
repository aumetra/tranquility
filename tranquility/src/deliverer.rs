use {
    crate::error::Error,
    itertools::Itertools,
    reqwest::{Client, Request, Result as ReqwestResult},
    serde_json::Value,
    std::sync::Arc,
    tranquility_types::activitypub::{Activity, PUBLIC_IDENTIFIER},
};

fn prepare_request(client: &Client, url: &str, activity: &Value) -> ReqwestResult<Request> {
    client
        .post(url)
        .header("Content-Type", "application/activity+json")
        .json(activity)
        .build()
}

pub fn deliver(activity: Activity) -> Result<(), Error> {
    let activity_value = Arc::new(serde_json::to_value(&activity)?);

    let recipient_list = activity
        .to
        .into_iter()
        .merge(activity.cc)
        .unique()
        .filter(|url| url != PUBLIC_IDENTIFIER)
        .collect_vec();

    let client = &crate::REQWEST_CLIENT;
    for url in recipient_list {
        let activity_value = Arc::clone(&activity_value);

        tokio::spawn(async move {
            let remote_actor = match crate::fetcher::fetch_actor(&url).await {
                Ok(actor) => actor,
                Err(err) => {
                    warn!("Couldn't fetch actor: {}", err);
                    return;
                }
            };
            let request = match prepare_request(&client, &remote_actor.id, &activity_value) {
                Ok(request) => request,
                Err(err) => {
                    warn!("Couldn't prepare request: {}", err);
                    return;
                }
            };

            match client.execute(request).await {
                Ok(_) => (),
                Err(err) => warn!("Couldn't deliver activity: {}", err),
            }
        });
    }

    Ok(())
}
