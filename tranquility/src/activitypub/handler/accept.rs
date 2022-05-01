use crate::{activitypub::FollowActivity, database::Object, error::Error, state::ArcState};
use http::StatusCode;
use ormx::Table;
use tranquility_types::activitypub::Activity;

pub async fn handle(state: &ArcState, activity: Activity) -> Result<StatusCode, Error> {
    let follow_activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;
    let mut follow_activity_db = Object::by_url(&state.db_pool, follow_activity_url).await?;

    let mut follow_activity: FollowActivity = serde_json::from_value(follow_activity_db.data)?;
    // Check if the person rejecting the follow is actually the followed person
    if &activity.actor != follow_activity.activity.object.as_url().unwrap() {
        return Err(Error::Unauthorized);
    }

    if follow_activity.activity.r#type != "Follow" {
        return Err(Error::UnknownActivity);
    }

    follow_activity.approved = true;

    // Update the activity
    let follow_activity_value = serde_json::to_value(&follow_activity)?;
    follow_activity_db.data = follow_activity_value;
    follow_activity_db.update(&state.db_pool).await?;

    Ok(StatusCode::OK)
}
