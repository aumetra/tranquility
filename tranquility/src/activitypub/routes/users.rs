use {
    crate::{database::Actor, state::ArcState, unrejectable_err},
    ormx::Table,
    uuid::Uuid,
    warp::{reply::Response, Rejection, Reply},
};

pub async fn users(uuid: Uuid, state: ArcState) -> Result<Response, Rejection> {
    let actor = unrejectable_err!(Actor::get(&state.db_pool, uuid).await);

    Ok(warp::reply::json(&actor.actor).into_response())
}
