use {
    crate::{activitypub::fetcher, error::Error, state::ArcState},
    tranquility_types::activitypub::{activity::ObjectField, Activity, Object},
    uuid::Uuid,
    warp::http::StatusCode,
};

async fn insert_object(state: &ArcState, activity: &Activity) -> Result<Object, Error> {
    let (_owner, owner_db) = fetcher::fetch_actor(state, &activity.actor).await?;

    let mut object = activity.object.as_object().unwrap().to_owned();
    crate::activitypub::clean_object(&mut object);

    let object_value = serde_json::to_value(&object)?;

    crate::database::object::insert(&state.db_pool, Uuid::new_v4(), owner_db.id, object_value)
        .await?;

    Ok(object)
}

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<StatusCode, Error> {
    // Save the object in the database
    match activity.object {
        ObjectField::Object(_) => {
            let object = insert_object(state, &activity).await?;

            activity.object = ObjectField::Url(object.id);
        }
        ObjectField::Url(ref url) => {
            fetcher::fetch_object(state, url).await?;
        }
        ObjectField::Actor(_) => return Err(Error::UnknownActivity),
    }

    Ok(StatusCode::CREATED)
}
