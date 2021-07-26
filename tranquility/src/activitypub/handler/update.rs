use {
    crate::{
        activitypub::{fetcher, Clean},
        database::Actor,
        error::Error,
    },
    ormx::Table,
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Update activities are usually only used to update the actor
    // (For example, when the user changes their bio or display name)
    let ap_actor = activity
        .object
        .as_mut_actor()
        .ok_or(Error::UnknownActivity)?;
    ap_actor.clean();

    // Fetch the actor (just in case)
    fetcher::fetch_actor(ap_actor.id.as_str()).await?;

    let state = crate::state::get();
    let mut actor = Actor::by_url(&state.db_pool, ap_actor.id.as_str()).await?;

    // Update the actor value
    let ap_actor = serde_json::to_value(ap_actor)?;
    actor.actor = ap_actor;

    actor.update(&state.db_pool).await?;

    Ok(StatusCode::CREATED)
}
