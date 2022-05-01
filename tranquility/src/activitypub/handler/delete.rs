use crate::{activitypub::fetcher, database::Object, error::Error, state::ArcState};
use http::StatusCode;
use tranquility_types::activitypub::{activity::ObjectField, Activity};

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

    Object::delete_by_url(&state.db_pool, &object.id).await?;

    Ok(StatusCode::CREATED)
}
