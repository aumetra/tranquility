use {crate::error::Error, tranquility_types::activitypub::Activity, warp::http::StatusCode};

pub async fn handle(delete_activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = delete_activity
        .object
        .as_url()
        .ok_or(Error::UnknownActivity)?;

    let activity = crate::database::object::select::by_url(activity_url.as_ref()).await?;
    let activity: Activity = serde_json::from_value(activity.data)?;

    // Does the activity belong to the actor?
    if delete_activity.actor != activity.actor {
        return Err(Error::Unauthorized);
    }

    crate::database::object::delete::by_url(activity.id.as_ref()).await?;

    Ok(StatusCode::CREATED)
}
