use {
    crate::{activitypub::fetcher, database::Object, error::Error},
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize activity
    match activity.object {
        ObjectField::Actor(..) => return Err(Error::UnknownActivity),
        ObjectField::Object(..) => (),
        ObjectField::Url(ref url) => {
            let object = fetcher::fetch_object(url).await?;

            activity.object = ObjectField::Object(object);
        }
    }

    let object = activity.object.as_object().unwrap();

    let state = crate::state::get();
    Object::delete_by_url(&state.db_pool, &object.id).await?;

    Ok(StatusCode::CREATED)
}
