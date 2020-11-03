use {
    crate::{activitypub::fetcher, error::Error},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;

    // Fetch the activity (just in case)
    fetcher::fetch_activity(activity_url.as_ref()).await?;
    // Fetch the actor (just in case)
    fetcher::fetch_actor(activity.actor.as_ref()).await?;
    let actor = crate::database::actor::select::by_url(activity.actor.as_ref()).await?;

    let activity_value = serde_json::to_value(&activity)?;
    crate::database::object::insert(actor.id, &activity.id, activity_value).await?;

    Ok(StatusCode::CREATED)
}
