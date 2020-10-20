use crate::error::Error;
use http_signatures::HttpRequest;
use tranquility_types::activitypub::{Activity, Object};
use warp::{
    http::{HeaderMap, Method},
    path::FullPath,
    Rejection, Reply,
};

pub async fn verify_request(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: String,
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    activity: Activity,
) -> Result<Activity, Rejection> {
    let remote_actor = crate::fetcher::fetch_actor(activity.actor.clone())
        .await
        .map_err(|err| Error::from(err))?;

    let valid = tokio::task::spawn_blocking::<_, Result<bool, Error>>(move || {
        let public_key = remote_actor.public_key.public_key_pem.as_bytes();
        let query = if query.is_empty() {
            None
        } else {
            Some(query.as_str())
        };

        let request = HttpRequest::new(method.as_str(), path.as_str(), query, &headers);

        Ok(http_signatures::verify(request, public_key)?)
    })
    .await
    .unwrap()?;

    if valid {
        Ok(activity)
    } else {
        Err(Error::InvalidHttpSignature.into())
    }
}

pub async fn inbox(mut activity: Activity) -> Result<impl Reply, Rejection> {
    Ok("inbox")
}
