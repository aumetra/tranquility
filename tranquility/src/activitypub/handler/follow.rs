use {
    crate::{activitypub, error::Error},
    tranquility_types::activitypub::{activity::ObjectField, Activity},
    uuid::Uuid,
    warp::http::StatusCode,
};

pub async fn handle(mut activity: Activity) -> Result<StatusCode, Error> {
    let actor_url = match activity.object {
        ObjectField::Actor(ref actor) => actor.id.as_str(),
        ObjectField::Url(ref url) => url.as_str(),
        _ => return Err(Error::UnknownActivity),
    };

    // Fetch the actor (just in case)
    crate::fetcher::fetch_actor(actor_url).await?;
    let actor = crate::database::actor::select::by_url(actor_url).await?;

    // Normalize the activity
    if let ObjectField::Actor(actor) = activity.object {
        activity.object = ObjectField::Url(actor.id);
    }

    crate::database::activity::insert(actor.id, &activity).await?;

    let followed_url = activity.object.as_url().unwrap();
    let followed_actor = crate::database::actor::select::by_url(followed_url).await?;

    // Send out an accept activity if the followed actor is local
    if !followed_actor.remote {
        let accept_activity_id = Uuid::new_v4();
        let accept_activity = activitypub::create_activity(
            "Accept".into(),
            &accept_activity_id.to_hyphenated_ref().to_string(),
            followed_url,
            activity.id.clone(),
            vec![activity.actor.clone()],
            Vec::new(),
        );

        crate::database::activity::insert(followed_actor.id, &accept_activity).await?;

        crate::deliverer::deliver(activity)?;
    }

    Ok(StatusCode::CREATED)
}
