use crate::{database::Object, error::Error, state::ArcState};
use http::StatusCode;
use ormx::Delete;
use tranquility_types::activitypub::Activity;

pub async fn handle(state: &ArcState, activity: Activity) -> Result<StatusCode, Error> {
    let follow_activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;
    let follow_activity_db = Object::by_url(&state.db_pool, follow_activity_url).await?;
    let follow_activity: Activity = serde_json::from_value(follow_activity_db.data.clone())?;
    // Check if the person rejecting the follow is actually the followed person
    if &activity.actor != follow_activity.object.as_url().unwrap() {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    if follow_activity.r#type != "Follow" {
        return Err(Error::UnknownActivity);
    }

    follow_activity_db.delete(&state.db_pool).await?;

    Ok(StatusCode::OK)
}
