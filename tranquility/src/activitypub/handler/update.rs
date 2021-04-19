use {
    crate::{
        activitypub::{self, fetcher},
        database::Actor,
        error::Error,
        state::ArcState,
        unrejectable_err,
    },
    ormx::Table,
    tranquility_types::activitypub::Activity,
    warp::{http::StatusCode, reply::Response, Reply},
};

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<Response, Error> {
    // Update activities are usually only used to update the actor
    // (For example, when the user changes their bio or display name)
    let ap_actor = activity
        .object
        .as_mut_actor()
        .ok_or(Error::UnknownActivity)?;

    activitypub::clean_actor(ap_actor);

    // Fetch the actor (just in case)
    unrejectable_err!(fetcher::fetch_actor(state, ap_actor.id.as_str()).await);

    let mut actor = unrejectable_err!(Actor::by_url(&state.db_pool, ap_actor.id.as_str()).await);

    // Update the actor value
    let ap_actor = serde_json::to_value(ap_actor)?;
    actor.actor = ap_actor;

    unrejectable_err!(actor.update(&state.db_pool).await);

    Ok(StatusCode::CREATED.into_response())
}
