use crate::error::Error;
use tranquility_types::activitypub::Activity;
use warp::{http::HeaderValue, Rejection, Reply};

pub async fn verify_request(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: String,
    auth_header: HeaderValue,
    activity: Activity,
) -> Result<Activity, Rejection> {
    let remote_actor = crate::fetcher::fetch_actor(activity.actor.clone())
        .await
        .map_err(|err| Error::from(err))?;
    let public_key =
        crate::crypto::parse::public(remote_actor.public_key.public_key_pem.as_bytes())?;

    // TODO: Add HTTP signature verification

    Ok(activity)
}

pub async fn inbox(activity: Activity) -> Result<impl Reply, Rejection> {
    Ok("inbox")
}
