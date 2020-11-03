use {
    crate::error::Error,
    itertools::Itertools,
    reqwest::{
        header::{HeaderName, HeaderValue, DATE},
        Client, Request, Result as ReqwestResult,
    },
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

    let activity_id = activity.id;
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
            debug!("Delivering activity {} to actor {}...", activity_id, url);

            let (remote_actor, remote_actor_db) = match super::fetcher::fetch_actor(&url).await {
                Ok(actor) => actor,
                Err(err) => {
                    warn!("Couldn't fetch actor: {}", err);
                    return;
                }
            };
            let mut request = match prepare_request(client, &remote_actor.id, &activity_value) {
                Ok(request) => request,
                Err(err) => {
                    warn!("Couldn't prepare request: {}", err);
                    return;
                }
            };

            let date_header_value = match HeaderValue::from_str(&chrono::Utc::now().to_rfc2822()) {
                Ok(header_value) => header_value,
                Err(err) => {
                    warn!(
                        "Couldn't convert the current DateTime to HeaderValue: {}",
                        err
                    );
                    return;
                }
            };

            let activity_bytes = serde_json::to_vec(&activity_value).unwrap();
            let digest_header_value = match crate::crypto::digest::http_header(activity_bytes).await
            {
                Ok(header_value) => header_value,
                Err(err) => {
                    warn!("Couldn't calculate the HTTP digest header: {}", err);
                    return;
                }
            };

            request.headers_mut().insert(DATE, date_header_value);
            request
                .headers_mut()
                .insert(HeaderName::from_static("digest"), digest_header_value);

            let key_id = remote_actor.public_key.id;
            let private_key = remote_actor_db.private_key.unwrap();

            let (header_name, header_value) = {
                let request = request.try_clone().unwrap();
                let signature_result = tokio::task::spawn_blocking(move || {
                    http_signatures::sign(
                        &request,
                        &key_id,
                        &["(request-target)", "date", "digest"],
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
