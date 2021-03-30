use {
    crate::{
        database::model::{Actor as DbActor, Object as DbObject},
        error::Error,
        state::ArcState,
    },
    std::sync::Arc,
    tranquility_types::activitypub::{Activity, Actor},
};

pub async fn follow(state: &ArcState, db_actor: DbActor, followed: &Actor) -> Result<(), Error> {
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    // Check if there's already a follow activity
    // If there's already a follow activity, just say everything was successful
    let existing_follow_activity = crate::database::object::select::by_type_owner_and_object_url(
        &state.db_pool,
        "Follow",
        &db_actor.id,
        followed.id.as_str(),
    )
    .await;
    if existing_follow_activity.is_ok() {
        return Ok(());
    }

    let (follow_activity_id, follow_activity) = crate::activitypub::instantiate::activity(
        &state.config,
        "Follow",
        actor.id.as_str(),
        followed.id.clone(),
        vec![followed.id.clone()],
        vec![],
    );
    let follow_activity_value = serde_json::to_value(&follow_activity)?;

    crate::database::object::insert(
        &state.db_pool,
        follow_activity_id,
        db_actor.id,
        follow_activity_value,
    )
    .await?;

    crate::activitypub::deliverer::deliver(follow_activity, Arc::clone(state)).await?;

    Ok(())
}

pub async fn undo(state: &ArcState, db_actor: DbActor, db_activity: DbObject) -> Result<(), Error> {
    // Tried to delete someone else's activity
    if db_activity.owner_id != db_actor.id {
        return Err(Error::Unauthorized);
    }

    let activity: Activity = serde_json::from_value(db_activity.data)?;
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    // Send the undo activity to everyone who received the original activity
    let (undo_activity_id, undo_activity) = crate::activitypub::instantiate::activity(
        &state.config,
        "Undo",
        actor.id.as_str(),
        activity.id,
        activity.to,
        activity.cc,
    );
    let undo_activity_value = serde_json::to_value(&undo_activity)?;
    crate::database::object::insert(
        &state.db_pool,
        undo_activity_id,
        db_actor.id,
        undo_activity_value,
    )
    .await?;

    crate::activitypub::deliverer::deliver(undo_activity, Arc::clone(state)).await?;

    Ok(())
}

pub async fn unfollow(
    state: &ArcState,
    db_actor: DbActor,
    followed_db_actor: DbActor,
) -> Result<(), Error> {
    let followed_actor: Actor = serde_json::from_value(followed_db_actor.actor)?;

    let follow_activity = crate::database::object::select::by_type_owner_and_object_url(
        &state.db_pool,
        "Follow",
        &db_actor.id,
        followed_actor.id.as_str(),
    )
    .await?;

    undo(state, db_actor, follow_activity).await
}
