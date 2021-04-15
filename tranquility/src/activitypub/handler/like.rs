use {
    crate::{
        activitypub::fetcher,
        database::{Actor, InsertExt, InsertObject},
        error::Error,
        state::ArcState,
    },
    tranquility_types::activitypub::Activity,
    uuid::Uuid,
    warp::http::StatusCode,
};

pub async fn handle(state: &ArcState, activity: Activity) -> Result<StatusCode, Error> {
    let activity_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;

    // Fetch the activity (just in case)
    fetcher::fetch_activity(&state, &activity_url).await?;
    // Fetch the actor (just in case)
    fetcher::fetch_actor(&state, &activity.actor).await?;
    let actor = Actor::by_url(&state.db_pool, &activity.actor).await?;

    let activity_value = serde_json::to_value(&activity)?;

    InsertObject {
        id: Uuid::new_v4(),
        owner_id: actor.id,
        data: activity_value,
    }
    .insert(&state.db_pool)
    .await?;

    Ok(StatusCode::CREATED)
}
