use crate::error::Error;
use uuid::Uuid;
use warp::{Rejection, Reply};

pub async fn get_actor(uuid: String) -> Result<impl Reply, Rejection> {
    let uuid = Uuid::parse_str(&uuid).map_err(|err| Error::from(err))?;
    let actor = crate::database::actor::select::by_id(uuid).await?;

    Ok(warp::reply::json(&actor.actor))
}
