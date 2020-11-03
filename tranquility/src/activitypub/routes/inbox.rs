use {
    crate::{
        activitypub::{fetcher, handler},
        error::Error,
    },
    http_signatures::HttpRequest,
    tranquility_types::activitypub::Activity,
    warp::{
        http::{HeaderMap, Method},
        path::FullPath,
        Rejection, Reply,
    },
};

pub async fn verify_request(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: uuid::Uuid,
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    activity: Activity,
) -> Result<Activity, Rejection> {
    let (remote_actor, _remote_actor_db) = fetcher::fetch_actor(activity.actor.as_ref())
        .await
        .map_err(Error::from)?;

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
        Err(Error::Unauthorized.into())
    }
}

pub async fn inbox(activity: Activity) -> Result<impl Reply, Rejection> {
    let response = match activity.r#type.as_str() {
        "Accept" => handler::accept::handle(activity).await,
        "Create" => handler::create::handle(activity).await,
        "Delete" => handler::delete::handle(activity).await,
        "Follow" => handler::follow::handle(activity).await,
        "Like" => handler::like::handle(activity).await,
        "Reject" => handler::reject::handle(activity).await,
        "Undo" => handler::undo::handle(activity).await,
        "Update" => handler::update::handle(activity).await,
        _ => Err(Error::UnknownActivity),
    };

    response.map_err(Rejection::from)
}
