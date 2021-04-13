use {
    crate::{database::model::Actor, error::Error, state::ArcState},
    ormx::Table,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let actor = Actor::get(&state.db_pool, uuid)
        .await
        .map_err(Error::from)?;

    Ok(warp::reply::json(&actor.actor))
}
