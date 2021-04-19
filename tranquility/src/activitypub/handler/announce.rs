use {
    crate::{
        activitypub::fetcher,
        database::{Actor, InsertExt, InsertObject},
        error::Error,
        state::ArcState,
        unrejectable_err,
    },
    tranquility_types::activitypub::Activity,
    uuid::Uuid,
    warp::{http::StatusCode, reply::Response, Reply},
};

pub async fn handle(state: &ArcState, activity: Activity) -> Result<Response, Error> {
    let object_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;

    // Fetch the object (just in case)
    unrejectable_err!(fetcher::fetch_object(&state, &object_url).await);
    // Fetch the actor (just in case)
    unrejectable_err!(fetcher::fetch_actor(&state, &activity.actor).await);

    let actor = unrejectable_err!(Actor::by_url(&state.db_pool, &activity.actor).await);
    let activity_value = serde_json::to_value(&activity)?;

    unrejectable_err!(
        InsertObject {
            id: Uuid::new_v4(),
            owner_id: actor.id,
            data: activity_value,
        }
        .insert(&state.db_pool)
        .await
    );

    Ok(StatusCode::CREATED.into_response())
}
