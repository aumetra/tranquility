use {
    crate::{activitypub::fetcher, error::Error},
    tranquility_types::activitypub::{activity::ObjectField, Activity},
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

    let object = activity.object.as_object().unwrap();

    crate::database::object::delete::by_url(object.id.as_ref()).await?;
    crate::database::object::delete::by_object_url(object.id.as_ref()).await?;

    Ok(StatusCode::CREATED)
}
