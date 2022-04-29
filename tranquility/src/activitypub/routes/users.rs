use {
    crate::{database::Actor, state::ArcState},
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let actor = Actor::get(&state.db_pool, uuid).await?;

    Ok(warp::reply::json(&actor.actor))
}
