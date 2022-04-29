use crate::{activitypub::ActivityObject, database::Object, error::Error, state::ArcState};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use http::StatusCode;
use ormx::Table;
use tranquility_types::activitypub::IsPrivate;
use uuid::Uuid;

pub async fn objects(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
) -> Result<impl IntoResponse, Error> {
    let object = Object::get(&state.db_pool, id).await?;
    let activity_or_object: ActivityObject = serde_json::from_value(object.data.clone())?;

    // Do not expose private activities/object publicly
    if activity_or_object.is_private() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    Ok(Json(&object.data).into_response())
}
