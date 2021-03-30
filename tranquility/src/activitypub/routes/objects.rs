use {
    crate::{activitypub::ActivityObject, error::Error, state::ArcState},
    tranquility_types::activitypub::IsPrivate,
    uuid::Uuid,
    warp::{http::StatusCode, reply::Response, Rejection, Reply},
};

pub async fn objects(id: Uuid, state: ArcState) -> Result<Response, Rejection> {
    let object = crate::database::object::select::by_id(&state.db_pool, id).await?;
    let activity_or_object: ActivityObject =
        serde_json::from_value(object.data.clone()).map_err(Error::from)?;

    // Do not expose private activities/object publicly
    if activity_or_object.is_private() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    Ok(warp::reply::json(&object.data).into_response())
}
