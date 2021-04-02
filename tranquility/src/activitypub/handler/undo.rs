use {
    crate::{error::Error, state::ArcState},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(state: &ArcState, delete_activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = delete_activity
        .object
        .as_url()
        .ok_or(Error::UnknownActivity)?;

    let activity = crate::database::object::select::by_url(&state.db_pool, &activity_url).await?;
    let activity: Activity = serde_json::from_value(activity.data)?;

    crate::database::object::delete::by_url(&state.db_pool, &activity.id).await?;

    Ok(StatusCode::CREATED)
}
