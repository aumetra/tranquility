use {
    crate::{database::Object, error::Error},
    tranquility_types::activitypub::Activity,
    warp::http::StatusCode,
};

pub async fn handle(delete_activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = delete_activity
        .object
        .as_url()
        .ok_or(Error::UnknownActivity)?;

    let state = crate::state::get();
    Object::delete_by_url(&state.db_pool, activity_url).await?;

    Ok(StatusCode::CREATED)
}
