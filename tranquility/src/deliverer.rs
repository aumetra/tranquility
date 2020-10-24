use {
    crate::error::Error,
    itertools::Itertools,
    reqwest::{Client, Request, Result as ReqwestResult},
    serde_json::Value,
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
    let activity_value = serde_json::to_value(&activity)?;

    let recipient_list = activity
        .to
        .into_iter()
        .merge(activity.cc)
        .unique()
        .filter(|url| url != PUBLIC_IDENTIFIER)
        .collect_vec();

    let client = &crate::REQWEST_CLIENT;
    tokio::spawn(async move {
        for url in recipient_list {
            let (remote_actor, remote_actor_db) = match crate::fetcher::fetch_actor(&url).await {
                Ok(actor) => actor,
                Err(err) => {
                    warn!("Couldn't fetch actor: {}", err);
                    return;
                }
            };
            let mut request = match prepare_request(&client, &remote_actor.id, &activity_value) {
                Ok(request) => request,
                Err(err) => {
                    warn!("Couldn't prepare request: {}", err);
                    return;
                }
            };

            let key_id = remote_actor.public_key.id;
            let private_key = remote_actor_db.private_key.unwrap();

            let (header_name, header_value) = {
                let request = request.try_clone().unwrap();
                let signature_result = tokio::task::spawn_blocking(move || {
                    http_signatures::sign(
                        // This just takes a reference to the request and tuns it into an `HttpRequest` object
                        // Yes, it's ugly, but the `Request` type doesn't have something like `.as_ref()`
                        (&request).into(),
                        &key_id,
                        vec!["(request-target)"],
                        private_key.as_bytes(),
                    )
                })
                .await
                .unwrap();

                match signature_result {
                    Ok(val) => val,
                    Err(err) => {
                        warn!("Couldn't sign HTTP request: {}", err);
                        return;
                    }
                }
            };

            request.headers_mut().insert(header_name, header_value);

            match client.execute(request).await {
                Ok(_) => (),
                Err(err) => warn!("Couldn't deliver activity: {}", err),
            }
        }
    });

    Ok(())
}
