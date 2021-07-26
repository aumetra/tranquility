use {
    crate::{database::Actor, map_err},
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid) -> Result<impl Reply, Rejection> {
    let state = crate::state::get();
    let actor = map_err!(Actor::get(&state.db_pool, uuid).await)?;

    Ok(warp::reply::json(&actor.actor))
}
