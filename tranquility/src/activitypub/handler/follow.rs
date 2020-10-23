use {
    crate::error::Error,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    let actor_url = match activity.object {
        ObjectField::Actor(ref actor) => actor.id.as_str(),
        ObjectField::Url(ref url) => url.as_str(),
        _ => return Err(Error::UnknownActivity),
    };

    // Fetch the actor (just in case)
    crate::fetcher::fetch_actor(actor_url).await?;
    let actor = crate::database::actor::select::by_url(actor_url).await?;

    // Normalize the activity
    if let ObjectField::Actor(actor) = activity.object {
        activity.object = ObjectField::Url(actor.id);
    }

    crate::database::activity::insert(actor.id, &activity).await?;

    Ok(StatusCode::CREATED)
}
