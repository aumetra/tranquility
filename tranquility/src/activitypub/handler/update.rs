use {
    crate::{
        activitypub::{self, fetcher},
        error::Error,
        state::ArcState,
    },
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<StatusCode, Error> {
    // Update activities are usually only used to update the actor
    // (For example, when the user changes their bio or display name)
    let actor = activity
        .object
        .as_mut_actor()
        .ok_or(Error::UnknownActivity)?;

    activitypub::clean_actor(actor);

    // Fetch the actor (just in case)
    fetcher::fetch_actor(state, actor.id.as_str()).await?;

    crate::database::actor::update(&state.db_pool, actor).await?;

    Ok(StatusCode::CREATED)
}
