use {
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid) -> Result<impl Reply, Rejection> {
    let actor = crate::database::actor::select::by_id(uuid).await?;

    Ok(warp::reply::json(&actor.actor))
}
