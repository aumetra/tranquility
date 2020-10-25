use {
    crate::error::Error,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize the activity
    match activity.object {
        ObjectField::Object(object) => {
            crate::fetcher::fetch_object(&object.id).await?;

            activity.object = ObjectField::Url(object.id);
        }
        ObjectField::Url(ref url) => {
            // I know, I could just save the object into the database directly instead of refetching it
            crate::fetcher::fetch_object(url).await?;
        }
        ObjectField::Actor(_) => return Err(Error::UnknownActivity),
    }

    // Are they actually publishing the object for themselves?
    if activity.actor != activity.object.as_object().unwrap().attributed_to {
        return Err(Error::Unauthorized);
    }

    let object = activity.object.as_mut_object().unwrap();
    object.content = ammonia::clean(&object.content);

    let db_actor = crate::database::actor::select::by_url(activity.actor.as_ref()).await?;
    let activity_value = serde_json::to_value(&activity)?;
    crate::database::object::insert(db_actor.id, &activity.id, activity_value).await?;

    Ok(StatusCode::CREATED)
}
