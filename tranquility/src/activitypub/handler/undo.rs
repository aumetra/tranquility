use crate::{database::Object, error::Error, state::ArcState};
use http::StatusCode;
use tranquility_types::activitypub::Activity;

pub async fn handle(state: &ArcState, delete_activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = delete_activity
        .object
        .as_url()
        .ok_or(Error::UnknownActivity)?;

    Object::delete_by_url(&state.db_pool, activity_url).await?;

    Ok(StatusCode::CREATED)
}
