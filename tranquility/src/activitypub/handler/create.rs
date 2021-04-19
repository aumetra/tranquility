use {
    crate::{
        activitypub::fetcher,
        database::{InsertExt, InsertObject},
        error::Error,
        state::ArcState,
        unrejectable_err,
    },
    tranquility_types::activitypub::{activity::ObjectField, Activity, Object},
    uuid::Uuid,
    warp::{http::StatusCode, reply::Response, Reply},
};

async fn insert_object(state: &ArcState, activity: &Activity) -> anyhow::Result<Object> {
    let (_owner, owner_db) = fetcher::fetch_actor(state, &activity.actor).await?;

    let mut object = activity.object.as_object().unwrap().to_owned();
    crate::activitypub::clean_object(&mut object);

    let object_value = serde_json::to_value(&object)?;

    InsertObject {
        id: Uuid::new_v4(),
        owner_id: owner_db.id,
        data: object_value,
    }
    .insert(&state.db_pool)
    .await?;

    Ok(object)
}

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<Response, Error> {
    // Save the object in the database
    match activity.object {
        ObjectField::Object(_) => {
            let object = unrejectable_err!(insert_object(state, &activity).await);

            activity.object = ObjectField::Url(object.id);
        }
        ObjectField::Url(ref url) => {
            unrejectable_err!(fetcher::fetch_object(state, url).await);
        }
        ObjectField::Actor(_) => return Err(Error::UnknownActivity),
    }

    Ok(StatusCode::CREATED.into_response())
}
