use {
    crate::{activitypub::fetcher, error::Error},
    tranquility_types::activitypub::{activity::ObjectField, Activity, Object},
    warp::http::StatusCode,
};

async fn insert_object(activity: &Activity) -> Result<Object, Error> {
    let (_owner, owner_db) = fetcher::fetch_actor(&activity.actor).await?;

    let mut object = activity.object.as_object().unwrap().to_owned();
    crate::activitypub::clean_object(&mut object);

    let object_value = serde_json::to_value(&object)?;

    crate::database::object::insert(owner_db.id, object_value).await?;

    Ok(object)
}

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    // Normalize the activity
    match activity.object {
        ObjectField::Object(_) => {
            let object = insert_object(&activity).await?;

            activity.object = ObjectField::Url(object.id);
        }
        ObjectField::Url(ref url) => {
            fetcher::fetch_object(url).await?;
        }
        ObjectField::Actor(_) => return Err(Error::UnknownActivity),
    }

    fetcher::fetch_object(activity.object.as_url().unwrap()).await?;

    let db_actor = crate::database::actor::select::by_url(activity.actor.as_ref()).await?;
    let activity_value = serde_json::to_value(&activity)?;
    crate::database::object::insert(db_actor.id, activity_value).await?;

    Ok(StatusCode::CREATED)
}
