use {
    crate::state::ArcState,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn users(uuid: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let actor = crate::database::actor::select::by_id(&state.db_pool, uuid).await?;

    Ok(warp::reply::json(&actor.actor))
}
