use {
    crate::{
        activitypub::{
            fetcher, handler,
            routes::{custom_json_parser, optional_raw_query},
        },
        crypto,
        error::Error,
        state::ArcState,
    },
    core::ops::Not,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::{
        http::{HeaderMap, Method},
        path::FullPath,
        Filter, Rejection, Reply,
    },
};

pub fn validate_request(
    state: &ArcState,
) -> impl Filter<Extract = (Activity,), Error = Rejection> + Clone {
    crate::state::filter(state)
        .and(warp::method())
        .and(warp::path::full())
        .and(optional_raw_query())
        .and(warp::header::headers_cloned())
        .and(custom_json_parser())
        .and_then(verify_signature)
        .untuple_one()
        .and_then(verify_ownership)
}

async fn verify_ownership(state: ArcState, activity: Activity) -> Result<Activity, Rejection> {
    // It's fine if the objects or activities don't match in this case
    if activity.r#type == "Announce" || activity.r#type == "Follow" {
        return Ok(activity);
    }

    let identity_match = match activity.object {
        ObjectField::Actor(ref actor) => actor.id == activity.actor,
        ObjectField::Object(ref object) => object.attributed_to == activity.actor,
        ObjectField::Url(ref url) => {
            let entity = fetcher::fetch_any(&state, url).await?;
            entity.is_owned_by(activity.actor.as_str())
        }
    };

    identity_match
        .then(|| activity)
        .ok_or_else(|| Error::Unauthorized.into())
}

async fn verify_signature(
    state: ArcState,
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    activity: Activity,
) -> Result<(ArcState, Activity), Rejection> {
    let (remote_actor, _remote_actor_db) = fetcher::fetch_actor(&state, activity.actor.as_ref())
        .await
        .map_err(Error::from)?;

    let public_key = remote_actor.public_key.public_key_pem;
    let query = query.is_empty().not().then(|| query);

    crypto::request::verify(method, path, query, headers, public_key)
        .await?
        .then(|| (state, activity))
        .ok_or_else(|| Error::Unauthorized.into())
}

macro_rules! match_handler {
    {
        ($state:ident, $activity:ident);

        $($type:literal => $handler:expr),+
    } => {
        match $activity.r#type.as_str() {
            $(
                $type => $handler(&$state, $activity).await,
            )+
            _ => Err(crate::error::Error::UnknownActivity),
        }
    }
}

pub async fn inbox(
    // Do we even care about the user ID?
    // Theoretically we could just use one shared inbox and get rid of the unique inboxes
    _user_id: uuid::Uuid,
    state: ArcState,
    activity: Activity,
) -> Result<impl Reply, Rejection> {
    let response = match_handler! {
        (state, activity);

        "Accept" => handler::accept::handle,
        "Create" => handler::create::handle,
        "Delete" => handler::delete::handle,
        "Follow" => handler::follow::handle,
        "Like" => handler::like::handle,
        "Reject" => handler::reject::handle,
        "Undo" => handler::undo::handle,
        "Update" => handler::update::handle
    };

    response.map_err(Rejection::from)
}
