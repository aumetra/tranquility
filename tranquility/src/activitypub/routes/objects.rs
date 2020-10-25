use {
    crate::error::Error,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn objects(id: String) -> Result<impl Reply, Rejection> {
    let id = Uuid::parse_str(&id).map_err(Error::from)?;

    let object = crate::database::object::select::by_id(id).await?;

    Ok(warp::reply::json(&object.data))
}
