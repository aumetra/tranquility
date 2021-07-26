use {
    crate::{
        activitypub::{
            fetcher,
            routes::{ap_json, optional_raw_query},
        },
        crypto,
        error::Error,
        map_err, match_handler,
    },
    core::ops::Not,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::{
        http::{HeaderMap, Method},
        path::FullPath,
        Filter, Rejection, Reply,
    },
};

/// Return a filter that
/// - Decodes the activity
/// - Verifies the HTTP signature
/// - Checks if the activity/object contained/referenced in the activity actually belongs to the author of the activity
pub fn validate_request() -> impl Filter<Extract = (Activity,), Error = Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(optional_raw_query())
        .and(warp::header::headers_cloned())
        .and(ap_json())
        .and_then(verify_signature)
        .and_then(verify_ownership)
}

/// Checks if the activity/object contained/referenced in the activity actually belongs to the author of the activity
async fn verify_ownership(activity: Activity) -> Result<Activity, Rejection> {
    // It's fine if the objects or activities don't match in this case
    if activity.r#type == "Announce" || activity.r#type == "Follow" {
        return Ok(activity);
    }

    let identity_match = match activity.object {
        ObjectField::Actor(ref actor) => actor.id == activity.actor,
        ObjectField::Object(ref object) => object.attributed_to == activity.actor,
        ObjectField::Url(ref url) => {
            let entity = fetcher::fetch_any(url).await?;
            entity.is_owned_by(activity.actor.as_str())
        }
    };

    identity_match
        .then(|| activity)
        .ok_or_else(|| Error::Unauthorized.into())
}

/// Verifies the HTTP signature with the public key of the owner of the activity
async fn verify_signature(
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    activity: Activity,
) -> Result<Activity, Rejection> {
    let (remote_actor, _remote_actor_db) = map_err!(fetcher::fetch_actor(&activity.actor).await)?;

    let public_key = remote_actor.public_key.public_key_pem;
    let query = query.is_empty().not().then(|| query);

    crypto::request::verify(method, path, query, headers, public_key)
        .await?
        .then(|| activity)
        .ok_or_else(|| Error::Unauthorized.into())
}

/// Inbox handler
pub async fn inbox(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: uuid::Uuid,
    activity: Activity,
) -> Result<impl Reply, Rejection> {
    let response = match_handler! {
        activity;

        Accept,
        Announce,
        Create,
        Delete,
        Follow,
        Like,
        Reject,
        Undo,
        Update
    };

    response.map_err(Rejection::from)
}
