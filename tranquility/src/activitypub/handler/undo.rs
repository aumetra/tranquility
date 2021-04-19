use {
    crate::{database::Object, error::Error, state::ArcState, unrejectable_err},
    tranquility_types::activitypub::Activity,
    warp::{http::StatusCode, reply::Response, Reply},
};

pub async fn handle(state: &ArcState, delete_activity: Activity) -> Result<Response, Error> {
    let activity_url = delete_activity
        .object
        .as_url()
        .ok_or(Error::UnknownActivity)?;

    unrejectable_err!(Object::delete_by_url(&state.db_pool, activity_url).await);

    Ok(StatusCode::CREATED.into_response())
}
