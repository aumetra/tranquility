use {
    crate::{activitypub::fetcher, error::Error, state::ArcState},
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::http::StatusCode,
};

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize activity
    match activity.object {
        ObjectField::Actor(..) => return Err(Error::UnknownActivity),
        ObjectField::Object(..) => (),
        ObjectField::Url(ref url) => {
            let object = fetcher::fetch_object(state, url).await?;

            activity.object = ObjectField::Object(object);
        }
    }

    let object = activity.object.as_object().unwrap();

    crate::database::object::delete::by_url(&state.db_pool, &object.id).await?;

    Ok(StatusCode::CREATED)
}
