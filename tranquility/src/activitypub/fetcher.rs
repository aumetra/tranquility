use {
    crate::{
        attempt_fetch,
        database::{Actor as DbActor, InsertActor, InsertExt, InsertObject, Object as DbObject},
        error::Error,
        impl_from, impl_into, impl_is_owned_by, map_err,
        state::ArcState,
    },
    reqwest::IntoUrl,
    serde_json::Value,
    std::fmt::Debug,
    tranquility_types::activitypub::{activity::ObjectField, Activity, Actor, Object},
    uuid::Uuid,
};

pub enum Entity {
    Activity(Activity),
    Actor(Actor),
    Object(Object),
}

impl_from!(Entity; Activity, Actor, Object);
impl_into!(Entity; Activity, Actor, Object);
impl_is_owned_by!(
    Entity;
    (Activity, actor),
    (Actor, id),
    (Object, attributed_to)
);

/// Takes any URL that points to an ActivityPub entity  
///
/// It makes multiple attempts to fetch the entity and decode it into different normalized forms.
/// If none of the attempts succeed, a `Fetch` error is returned
#[instrument(skip(state))]
pub async fn fetch_any(state: &ArcState, url: &str) -> Result<Entity, Error> {
    // Create a custom closure around the `fetch_actor` function
    // Otherwise the pattern in the macro won't match
    let fetch_actor_fn = |state, url| async move {
        fetch_actor(state, url)
            .await
            .map(|(actor, _db_actor)| actor)
    };

    attempt_fetch!(state, url, [fetch_activity, fetch_actor_fn, fetch_object]);

    Err(Error::Fetch)
}

/// Attempt to deserialize the data from the given URL as an ActivityPub activity
#[instrument(skip(state))]
pub async fn fetch_activity(state: &ArcState, url: &str) -> Result<Activity, Error> {
    debug!("Fetching remote actor...");

    match DbObject::by_url(&state.db_pool, url).await {
        Ok(activity) => return Ok(serde_json::from_value(activity.data)?),
        Err(e) => debug!(
            error = ?e,
            "Activity not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Activity(mut activity) = fetch_entity(url).await? {
        let (_actor, actor_db) = fetch_actor(state, &activity.actor).await?;
        // Normalize the activity
        if let Some(object) = activity.object.as_mut_object() {
            crate::activitypub::clean_object(object);

            let object_value = serde_json::to_value(&object)?;
            InsertObject {
                id: Uuid::new_v4(),
                owner_id: actor_db.id,
                data: object_value,
            }
            .insert(&state.db_pool)
            .await?;

            activity.object = ObjectField::Url(object.id.to_owned());
        } else if activity.object.as_actor().is_some() {
            return Err(Error::UnknownActivity);
        }

        let activity_value = serde_json::to_value(&activity)?;

        InsertObject {
            id: Uuid::new_v4(),
            owner_id: actor_db.id,
            data: activity_value,
        }
        .insert(&state.db_pool)
        .await?;

        Ok(activity)
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

/// Attempt to deserialize the data from the given URL as an ActivityPub actor
#[instrument(skip(state))]
pub async fn fetch_actor(state: &ArcState, url: &str) -> Result<(Actor, DbActor), Error> {
    debug!("Fetching remote actor...");

    match DbActor::by_url(&state.db_pool, url).await {
        Ok(actor) => return Ok((serde_json::from_value(actor.actor.clone())?, actor)),
        Err(e) => debug!(
            error = ?e,
            "Actor not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Actor(mut actor) = fetch_entity(url).await? {
        crate::activitypub::clean_actor(&mut actor);

        let actor_value = map_err!(serde_json::to_value(&actor))?;
        let db_actor = InsertActor {
            id: Uuid::new_v4(),
            username: actor.username.clone(),
            email: None,
            password_hash: None,
            actor: actor_value,
            private_key: None,
            remote: true,
        }
        .insert(&state.db_pool)
        .await?;

        Ok((actor, db_actor))
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

/// Attempt to deserialize the data from the given URL as an ActivityPub object
#[instrument(skip(state))]
pub async fn fetch_object(state: &ArcState, url: &str) -> Result<Object, Error> {
    debug!("Fetching remote object...");

    match DbObject::by_url(&state.db_pool, url).await {
        Ok(object) => return Ok(serde_json::from_value(object.data)?),
        Err(e) => debug!(
            error = ?e,
            "Object not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Object(mut object) = fetch_entity(url).await? {
        crate::activitypub::clean_object(&mut object);

        let (_actor, actor_db) = fetch_actor(state, &object.attributed_to).await?;

        let object_value = serde_json::to_value(&object)?;

        InsertObject {
            id: Uuid::new_v4(),
            owner_id: actor_db.id,
            data: object_value,
        }
        .insert(&state.db_pool)
        .await?;

        Ok(object)
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

/// Fetch the contents from the URL and attempt to parse them as different ActivityPub types
/// until either some type works or none of them work
#[instrument]
async fn fetch_entity<T: Debug + IntoUrl + Send>(url: T) -> Result<Entity, Error> {
    let client = &crate::util::HTTP_CLIENT;
    let request = client
        .get(url)
        .header(
            "Accept",
            "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
        )
        .build()?;

    let entity: Value = client.execute(request).await?.json().await?;

    let entity = if entity["type"].as_str().unwrap() == "Person" {
        // This should be deserializable into an actor
        let actor = serde_json::from_value(entity)?;

        Entity::Actor(actor)
    } else if entity.get("object").is_some() {
        // This should be deserializable into an activity
        let activity = serde_json::from_value(entity)?;

        Entity::Activity(activity)
    } else {
        // This could be deserializable into an object (but could also be nothing)
        let object = serde_json::from_value(entity)?;

        Entity::Object(object)
    };

    Ok(entity)
}
