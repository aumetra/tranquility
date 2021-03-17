use {
    crate::{database::model::Actor as DbActor, error::Error, state::ArcState},
    paste::paste,
    reqwest::IntoUrl,
    serde_json::Value,
    std::fmt::Debug,
    tranquility_types::activitypub::{activity::ObjectField, Activity, Actor, Object},
    uuid::Uuid,
};

macro_rules! impl_from {
    ($enum:ty; $($type:ident),+) => {
        $(
            impl From<$type> for $enum {
                fn from(value: $type) -> Self {
                    Self::$type(value)
                }
            }
        )+
    }
}

macro_rules! impl_into {
    ($enum:ty; $($type:ident),+) => {
        paste! {
            impl $enum {
                $(
                    #[allow(dead_code)]
                    pub fn [<into_ $type:lower>](self) -> Option<$type> {
                        match self {
                            Self::$type(val) => Some(val),
                            _ => None,
                        }
                    }
                )+
            }
        }
    }
}

macro_rules! impl_is_owned_by {
    ($enum:ty; $(($branch:ident, $field:ident)),+) => {
        impl $enum {
            pub fn is_owned_by(&self, actor_id: &str) -> bool {
                match self {
                    $(
                        Self::$branch(val) => val.$field == actor_id,
                    )+
                }
            }
        }
    }
}

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

/// Try fetching an something that can be turned into an `Entity` via the given methods  
/// If the fetch succeeds, the function returns with the success value  
/// If it doesn't, the error gets logged and the function continues  
macro_rules! attempt_fetch {
    ($state:ident, $url:ident, [$($func:ident),+]) => {{
        $(
            match $func($state, $url).await {
                Ok(val) => return Ok(val.into()),
                Err(err) => debug!(error = ?err, "Couldn't fetch entity"),
            }
        )+
    }};
}

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

#[instrument(skip(state))]
pub async fn fetch_activity(state: &ArcState, url: &str) -> Result<Activity, Error> {
    debug!("Fetching remote actor...");

    match crate::database::object::select::by_url(&state.db_pool, url).await {
        Ok(activity) => return Ok(serde_json::from_value(activity.data)?),
        Err(e) => debug!(
            error = ?e,
            "Activity not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Activity(mut activity) = fetch_entity(url).await? {
        let (_actor, actor_db) = fetch_actor(state, activity.actor.as_ref()).await?;
        // Normalize the activity
        if let Some(object) = activity.object.as_mut_object() {
            crate::activitypub::clean_object(object);

            let object_value = serde_json::to_value(&object)?;
            crate::database::object::insert(
                &state.db_pool,
                Uuid::new_v4(),
                actor_db.id,
                object_value,
            )
            .await?;

            activity.object = ObjectField::Url(object.id.to_owned());
        } else if activity.object.as_actor().is_some() {
            return Err(Error::UnknownActivity);
        }

        let activity_value = serde_json::to_value(&activity)?;
        crate::database::object::insert(
            &state.db_pool,
            Uuid::new_v4(),
            actor_db.id,
            activity_value,
        )
        .await?;

        Ok(activity)
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

#[instrument(skip(state))]
pub async fn fetch_actor(state: &ArcState, url: &str) -> Result<(Actor, DbActor), Error> {
    debug!("Fetching remote actor...");

    match crate::database::actor::select::by_url(&state.db_pool, url).await {
        Ok(actor) => return Ok((serde_json::from_value(actor.actor.clone())?, actor)),
        Err(e) => debug!(
            error = ?e,
            "Actor not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Actor(mut actor) = fetch_entity(url).await? {
        crate::activitypub::clean_actor(&mut actor);

        let db_actor =
            crate::database::actor::insert::remote(&state.db_pool, actor.username.as_ref(), &actor)
                .await?;

        Ok((actor, db_actor))
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

#[instrument(skip(state))]
pub async fn fetch_object(state: &ArcState, url: &str) -> Result<Object, Error> {
    debug!("Fetching remote object...");

    match crate::database::object::select::by_url(&state.db_pool, url).await {
        Ok(object) => return Ok(serde_json::from_value(object.data)?),
        Err(e) => debug!(
            error = ?e,
            "Object not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Object(mut object) = fetch_entity(url).await? {
        crate::activitypub::clean_object(&mut object);

        let (_actor, actor_db) = fetch_actor(state, object.attributed_to.as_ref()).await?;

        let object_value = serde_json::to_value(&object)?;
        crate::database::object::insert(&state.db_pool, Uuid::new_v4(), actor_db.id, object_value)
            .await?;

        Ok(object)
    } else {
        debug!("Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

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
