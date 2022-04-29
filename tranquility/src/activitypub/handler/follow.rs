use {
    crate::{
        activitypub::{self, deliverer, fetcher, FollowActivity},
        database::{Actor, InsertExt, InsertObject},
        error::Error,
        state::ArcState,
    },
    http::StatusCode,
    std::sync::Arc,
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    uuid::Uuid,
};

pub async fn handle(state: &ArcState, mut activity: Activity) -> Result<StatusCode, Error> {
    let actor_url = match activity.object {
        ObjectField::Actor(ref actor) => actor.id.as_str(),
        ObjectField::Url(ref url) => url.as_str(),
        ObjectField::Object(_) => return Err(Error::UnknownActivity),
    };

    // Fetch the actor (just in case)
    let (actor, actor_db) = fetcher::fetch_actor(state, actor_url).await?;

    // Normalize the activity
    if let ObjectField::Actor(actor) = activity.object {
        activity.object = ObjectField::Url(actor.id);
    }

    // Automatically approve the follow (this will be choice based at some point)
    let follow_activity = FollowActivity {
        activity,
        approved: true,
    };
    let activity = serde_json::to_value(&follow_activity)?;

    InsertObject {
        id: Uuid::new_v4(),
        owner_id: actor_db.id,
        data: activity,
    }
    .insert(&state.db_pool)
    .await?;

    let followed_url = follow_activity.activity.object.as_url().unwrap();
    let followed_actor = Actor::by_url(&state.db_pool, followed_url).await?;

    // Send out an accept activity if the followed actor is local
    if follow_activity.approved {
        let (accept_activity_id, accept_activity) = activitypub::instantiate::activity(
            &state.config,
            "Accept",
            followed_url,
            follow_activity.activity.id,
            vec![actor.id],
            Vec::new(),
        );
        let accept_activity_value = serde_json::to_value(&accept_activity)?;

        InsertObject {
            id: accept_activity_id,
            owner_id: followed_actor.id,
            data: accept_activity_value,
        }
        .insert(&state.db_pool)
        .await?;

        deliverer::deliver(accept_activity, Arc::clone(state)).await?;
    }

    Ok(StatusCode::CREATED)
}
