use {
    crate::{
        database::{Actor as DbActor, InsertExt, InsertObject, Object as DbObject},
        error::Error,
    },
    tranquility_types::activitypub::{Activity, Actor},
};

/// Create an Follow activity for a follow, save it and send it out
pub async fn follow(db_actor: DbActor, followed: &Actor) -> Result<(), Error> {
    let state = crate::state::get();
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    // Check if there's already a follow activity
    // If there's already a follow activity, just say everything was successful
    let existing_follow_activity = DbObject::by_type_owner_and_object_url(
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

    InsertObject {
        id: follow_activity_id,
        owner_id: db_actor.id,
        data: follow_activity_value,
    }
    .insert(&state.db_pool)
    .await?;

    crate::activitypub::deliverer::deliver(follow_activity).await?;

    Ok(())
}

/// Create an Undo activity for the given activity, save it and send it out
pub async fn undo(db_actor: DbActor, db_activity: DbObject) -> Result<(), Error> {
    let state = crate::state::get();

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

    InsertObject {
        id: undo_activity_id,
        owner_id: db_actor.id,
        data: undo_activity_value,
    }
    .insert(&state.db_pool)
    .await?;

    crate::activitypub::deliverer::deliver(undo_activity).await?;

    Ok(())
}

/// Search the follow activity in the database and undo it
pub async fn unfollow(db_actor: DbActor, followed_db_actor: DbActor) -> Result<(), Error> {
    let state = crate::state::get();
    let followed_actor: Actor = serde_json::from_value(followed_db_actor.actor)?;

    let follow_activity = DbObject::by_type_owner_and_object_url(
        &state.db_pool,
        "Follow",
        &db_actor.id,
        followed_actor.id.as_str(),
    )
    .await?;

    undo(db_actor, follow_activity).await
}
