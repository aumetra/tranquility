use {
    crate::{activitypub::ActivityObject, error::Error},
    tranquility_types::activitypub::IsPrivate,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn objects(id: Uuid) -> Result<impl Reply, Rejection> {
    let object = crate::database::object::select::by_id(id).await?;
    let activity_or_object: ActivityObject =
        serde_json::from_value(object.data.clone()).map_err(Error::from)?;

    if activity_or_object.is_private() {
        return Err(warp::reject::not_found());
    }

    Ok(warp::reply::json(&object.data))
}
