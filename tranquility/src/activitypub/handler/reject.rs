use {crate::error::Error, tranquility_types::activitypub::Activity, warp::http::StatusCode};

pub async fn handle(activity: Activity) -> Result<StatusCode, Error> {
    let follow_activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;
    let follow_activity_db = crate::database::object::select::by_url(follow_activity_url).await?;
    let follow_activity: Activity = serde_json::from_value(follow_activity_db.data)?;
    // Check if the person rejecting the follow is actually the followed person
    if &activity.actor != follow_activity.object.as_url().unwrap() {
        return Err(Error::Unauthorized);
    }

    if follow_activity.r#type != "Follow" {
        return Err(Error::UnknownActivity);
    }

    crate::database::object::delete::by_id(follow_activity_db.id).await?;

    Ok(StatusCode::OK)
}
