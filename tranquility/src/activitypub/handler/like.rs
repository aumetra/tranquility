use {
    crate::{
        activitypub::fetcher,
        database::{Actor, InsertExt, InsertObject},
        error::Error,
    },
    tranquility_types::activitypub::Activity,
    uuid::Uuid,
    warp::http::StatusCode,
};

pub async fn handle(activity: Activity) -> Result<StatusCode, Error> {
    let object_url = activity.object.as_url().ok_or(Error::UnknownActivity)?;

    // Fetch the object (just in case)
    fetcher::fetch_object(&object_url).await?;
    // Fetch the actor (just in case)
    fetcher::fetch_actor(&activity.actor).await?;

    let state = crate::state::get();
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
