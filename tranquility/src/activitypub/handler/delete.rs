use {
    crate::{
        activitypub::fetcher, database::Object, error::Error, state::ArcState, unrejectable_err,
    },
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    warp::{http::StatusCode, reply::Response, Reply},
};

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<Response, Error> {
    // Normalize activity
    match activity.object {
        ObjectField::Actor(..) => return Err(Error::UnknownActivity),
        ObjectField::Object(..) => (),
        ObjectField::Url(ref url) => {
            let object = unrejectable_err!(fetcher::fetch_object(state, url).await);

            activity.object = ObjectField::Object(object);
        }
    }

    let object = activity.object.as_object().unwrap();

    unrejectable_err!(Object::delete_by_url(&state.db_pool, &object.id).await);

    Ok(StatusCode::CREATED.into_response())
}
