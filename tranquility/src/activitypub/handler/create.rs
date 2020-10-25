use {
    crate::error::Error,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize the activity
    if let ObjectField::Url(url) = activity.object {
        let object = crate::fetcher::fetch_entity(url.as_str())
            .await?
            .into_object()
            .ok_or(Error::Fetch)?;

        activity.object = ObjectField::Object(object);
    }

    // Are they actually publishing the object for themselves?
    if activity.actor != activity.object.as_object().unwrap().attributed_to {
        return Err(Error::Unauthorized);
    }

    let object = activity.object.as_mut_object().unwrap();
    object.content = ammonia::clean(&object.content);

    let db_actor = crate::database::actor::select::by_url(activity.actor.as_ref()).await?;
    crate::database::activity::insert(db_actor.id, &activity).await?;

    Ok(StatusCode::CREATED)
}
