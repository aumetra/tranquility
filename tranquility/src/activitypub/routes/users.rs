use {
    crate::{database::Actor, map_err, state::ArcState},
    ormx::Table,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let actor = map_err!(Actor::get(&state.db_pool, uuid).await)?;

    Ok(warp::reply::json(&actor.actor))
}
