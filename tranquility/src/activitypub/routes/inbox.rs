use {
    super::{custom_json_type, optional_raw_query},
    crate::{
        activitypub::{
            fetcher::{self, Entity},
            handler,
        },
        crypto,
        error::Error,
    },
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::{
        http::{HeaderMap, Method},
        path::FullPath,
        Filter, Rejection, Reply,
    },
};

pub fn validate_request() -> impl Filter<Extract = (Activity,), Error = Rejection> + Copy {
    warp::method()
        .and(warp::path::full())
        .and(optional_raw_query())
        .and(warp::header::headers_cloned())
        .and(custom_json_type())
        .and_then(verify_signature)
        .and_then(verify_identity)
}

async fn verify_identity(activity: Activity) -> Result<Activity, Rejection> {
    // It's fine if the objects or activities don't match in this case
    if activity.r#type == "Announce" || activity.r#type == "Follow" {
        return Ok(activity);
    }

    let identity_match = match activity.object {
        ObjectField::Actor(ref actor) => actor.id == activity.actor,
        ObjectField::Object(ref object) => object.attributed_to == activity.actor,
        ObjectField::Url(ref url) => match fetcher::fetch_any(url).await? {
            Entity::Activity(ref_activity) => ref_activity.actor == activity.actor,
            Entity::Object(ref_object) => ref_object.attributed_to == activity.actor,
            Entity::Actor(ref_actor) => ref_actor.id == activity.actor,
        },
    };

    if identity_match {
        Ok(activity)
    } else {
        Err(Error::Unauthorized.into())
    }
}

async fn verify_signature(
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    activity: Activity,
) -> Result<Activity, Rejection> {
    let (remote_actor, _remote_actor_db) = fetcher::fetch_actor(activity.actor.as_ref())
        .await
        .map_err(Error::from)?;

    let public_key = remote_actor.public_key.public_key_pem;

    let query = if query.is_empty() { None } else { Some(query) };

    if crypto::request::verify(method, path, query, headers, public_key).await? {
        Ok(activity)
    } else {
        Err(Error::Unauthorized.into())
    }
}

pub async fn inbox(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: uuid::Uuid,
    activity: Activity,
) -> Result<impl Reply, Rejection> {
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
