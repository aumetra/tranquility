use {
    crate::{activitypub::fetcher, error::Error},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize activity
    match activity.object {
        ObjectField::Actor(_) => return Err(Error::UnknownActivity),
        ObjectField::Object(_) => (),
        ObjectField::Url(ref url) => {
            let object = fetcher::fetch_object(url).await?;

            activity.object = ObjectField::Object(object);
        }
    }

    let object = activity.object.as_object().ok_or(Error::UnknownActivity)?;
    // Does the object belong to the actor?
    if activity.actor != object.attributed_to {
        return Err(Error::Unauthorized);
    }

    crate::database::object::delete::by_url(object.id.as_ref()).await?;
    crate::database::object::delete::by_object_url(object.id.as_ref()).await?;

    Ok(StatusCode::CREATED)
}
