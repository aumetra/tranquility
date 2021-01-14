use {
    crate::{
        database::model::{Actor as DBActor, Object as DBObject},
        error::Error,
    },
    tranquility_types::activitypub::{Activity, Actor},
};

pub async fn follow(db_actor: DBActor, followed: &Actor) -> Result<(), Error> {
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    // Check if there's already a follow activity
    // If there's already a follow activity, just say everything was successful
    let existing_follow_activity = crate::database::object::select::by_type_owner_and_object_url(
        "Follow",
        &db_actor.id,
        followed.id.as_str(),
    )
    .await;
    if existing_follow_activity.is_ok() {
        return Ok(());
    }

    let (_follow_activity_id, follow_activity) = crate::activitypub::instantiate::activity(
        "Follow",
        actor.id.as_str(),
        followed.id.clone(),
        vec![followed.id.clone()],
        vec![],
    );
    let follow_activity_value = serde_json::to_value(&follow_activity)?;

    crate::database::object::insert(
        db_actor.id,
        follow_activity.id.as_str(),
        follow_activity_value,
    )
    .await?;

    crate::activitypub::deliverer::deliver(follow_activity).await?;

    Ok(())
}

pub async fn undo(db_actor: DBActor, db_activity: DBObject) -> Result<(), Error> {
    // Tried to delete someone else's activity
    if db_activity.owner_id != db_actor.id {
        return Err(Error::Unauthorized);
    }

    let activity: Activity = serde_json::from_value(db_activity.data)?;
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    // Send the undo activity to everyone who received the original activity
    let (_undo_activity_id, undo_activity) = crate::activitypub::instantiate::activity(
        "Undo",
        actor.id.as_str(),
        activity.id,
        activity.to,
        activity.cc,
    );
    let undo_activity_value = serde_json::to_value(&undo_activity)?;
    crate::database::object::insert(db_actor.id, undo_activity.id.as_str(), undo_activity_value)
        .await?;

    crate::activitypub::deliverer::deliver(undo_activity).await?;

    Ok(())
}

pub async fn unfollow(db_actor: DBActor, followed_db_actor: DBActor) -> Result<(), Error> {
    let followed_actor: Actor = serde_json::from_value(followed_db_actor.actor)?;

    let follow_activity = crate::database::object::select::by_type_owner_and_object_url(
        "Follow",
        &db_actor.id,
        followed_actor.id.as_str(),
    )
    .await?;

    undo(db_actor, follow_activity).await
}
