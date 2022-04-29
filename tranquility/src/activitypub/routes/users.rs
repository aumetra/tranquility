use crate::{database::Actor, error::Error, state::ArcState};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use uuid::Uuid;

pub async fn users(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
) -> Result<impl IntoResponse, Error> {
    let actor = Actor::get(&state.db_pool, id).await?;

    Ok(Json(&actor.actor))
}
