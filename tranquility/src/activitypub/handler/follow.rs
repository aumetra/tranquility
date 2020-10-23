use {
    crate::error::Error,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    let actor_url = match activity.object {
        ObjectField::Actor(ref actor) => actor.id.clone(),
        ObjectField::Url(ref url) => url.clone(),
        _ => return Err(Error::UnknownActivity),
    };

    // Fetch the actor (just in case)
    crate::fetcher::fetch_actor(actor_url.clone()).await?;
    let actor = crate::database::actor::select::by_url(actor_url.clone()).await?;

    // Normalize the activity
    if let ObjectField::Actor(actor) = activity.object {
        activity.object = ObjectField::Url(actor.id);
    }

    crate::database::activity::insert(actor.id, activity).await?;

    Ok(StatusCode::CREATED)
}
