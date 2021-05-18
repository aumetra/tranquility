use {
    crate::{activitypub::ActivityObject, database::Object, map_err, state::ArcState},
    ormx::Table,
    tranquility_types::activitypub::IsPrivate,
    uuid::Uuid,
    warp::{http::StatusCode, reply::Response, Rejection, Reply},
};

pub async fn objects(id: Uuid, state: ArcState) -> Result<Response, Rejection> {
    let object = map_err!(Object::get(&state.db_pool, id).await)?;
    let activity_or_object: ActivityObject = map_err!(serde_json::from_value(object.data.clone()))?;

    // Do not expose private activities/object publicly
    if activity_or_object.is_private() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    Ok(warp::reply::json(&object.data).into_response())
}
