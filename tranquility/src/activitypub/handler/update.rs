use {
    crate::{activitypub::fetcher, error::Error},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Update activities are usually only used to update the actor
    // (For example, when the user changes their bio or display name)
    let actor = activity
        .object
        .as_mut_actor()
        .ok_or(Error::UnknownActivity)?;
    // Is the sender actually who they say they are?
    if actor.id != activity.actor {
        return Err(Error::Unauthorized);
    }

    actor.name = ammonia::clean(&actor.name);
    actor.summary = ammonia::clean(&actor.summary);

    // Fetch the actor (just in case)
    fetcher::fetch_actor(actor.id.as_str()).await?;

    crate::database::actor::update(actor).await?;

    Ok(StatusCode::CREATED)
}
