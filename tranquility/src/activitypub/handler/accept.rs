use {
    crate::{activitypub::FollowActivity, error::Error},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(activity: Activity) -> Result<StatusCode, Error> {
    let follow_activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;
    let follow_activity_db = crate::database::object::select::by_url(follow_activity_url).await?;

    let mut follow_activity: FollowActivity = serde_json::from_value(follow_activity_db.data)?;
    // Check if the person rejecting the follow is actually the followed person
    if &activity.actor != follow_activity.activity.object.as_url().unwrap() {
        return Err(Error::Unauthorized);
    }

    if follow_activity.activity.r#type != "Follow" {
        return Err(Error::UnknownActivity);
    }

    follow_activity.approved = true;

    let follow_activity_value = serde_json::to_value(&follow_activity)?;
    crate::database::object::update(follow_activity_db.id, follow_activity_value)
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
