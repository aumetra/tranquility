use {
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn objects(id: Uuid) -> Result<impl Reply, Rejection> {
    let object = crate::database::object::select::by_id(id).await?;

    Ok(warp::reply::json(&object.data))
}
