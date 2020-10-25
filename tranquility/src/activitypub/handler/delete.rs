use {crate::error::Error, tranquility_types::activitypub::Activity, warp::http::StatusCode};

pub async fn handle(activity: Activity) -> Result<StatusCode, Error> {
    let object = activity.object.as_object().ok_or(Error::UnknownActivity)?;
    // Does the object belong to the actor?
    if activity.actor != object.attributed_to {
        return Err(Error::Unauthorized);
    }

    crate::database::object::delete::by_object_url(object.id.as_ref()).await?;

    Ok(StatusCode::CREATED)
}
