use {
    crate::{database::Object, error::Error, state::ArcState, unrejectable_err},
    ormx::Table,
    tranquility_types::activitypub::Activity,
    warp::{http::StatusCode, reply::Response, Reply},
};

pub async fn handle(state: &ArcState, activity: Activity) -> Result<Response, Error> {
    let follow_activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;
    let follow_activity_db =
        unrejectable_err!(Object::by_url(&state.db_pool, follow_activity_url).await);
    let follow_activity: Activity = serde_json::from_value(follow_activity_db.data.clone())?;
    // Check if the person rejecting the follow is actually the followed person
    if &activity.actor != follow_activity.object.as_url().unwrap() {
        return Err(Error::Unauthorized);
    }

    if follow_activity.r#type != "Follow" {
        return Err(Error::UnknownActivity);
    }

    unrejectable_err!(follow_activity_db.delete(&state.db_pool).await);

    Ok(StatusCode::OK.into_response())
}
