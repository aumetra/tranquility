use {
    crate::error::Error,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: String) -> Result<impl Reply, Rejection> {
    let uuid = Uuid::parse_str(&uuid).map_err(Error::from)?;
    let actor = crate::database::actor::select::by_id(uuid).await?;

    Ok(warp::reply::json(&actor.actor))
}
