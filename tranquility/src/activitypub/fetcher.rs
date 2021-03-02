use {
    crate::{database::model::Actor as DbActor, error::Error},
    reqwest::IntoUrl,
    serde_json::Value,
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

pub enum Entity {
    Activity(Activity),
    Actor(Actor),
    Object(Object),
}

impl_from!(Entity; Activity, Actor, Object);

// Keeping this for future use
#[allow(dead_code)]
impl Entity {
    pub fn into_activity(self) -> Option<Activity> {
        match self {
            Self::Activity(activity) => Some(activity),
            _ => None,
        }
    }

    pub fn into_actor(self) -> Option<Actor> {
        match self {
            Self::Actor(actor) => Some(actor),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<Object> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }
}

// This macro generates code that attempts to fetch the resource via the given function from the URL
// If the fetch succeeds, the function returns with the success value
// If it doesn't, the error gets logged and the function continues
macro_rules! attempt_fetch {
    ($func:ident, $url:ident) => {{
        match $func($url).await {
            Ok(val) => return Ok(val.into()),
            Err(err) => debug!(url = $url, error = ?err, "Couldn't fetch entity"),
        }
    }};
}

pub async fn fetch_any(url: &str) -> Result<Entity, Error> {
    // Create a custom closure around the `fetch_actor` function
    // Otherwise the pattern in the macro won't match
    let fetch_actor_fn =
        |url| async move { fetch_actor(url).await.map(|(actor, _db_actor)| actor) };

    attempt_fetch!(fetch_activity, url);
    attempt_fetch!(fetch_actor_fn, url);
    attempt_fetch!(fetch_object, url);

    Err(Error::Fetch)
}

pub async fn fetch_activity(url: &str) -> Result<Activity, Error> {
    debug!("Fetching remote actor...");

    match crate::database::object::select::by_url(url).await {
        Ok(activity) => return Ok(serde_json::from_value(activity.data)?),
        Err(e) => debug!(
            url,
            error = ?e,
            "Activity not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Activity(mut activity) = fetch_entity(url).await? {
        let (_actor, actor_db) = fetch_actor(activity.actor.as_ref()).await?;
        // Normalize the activity
        if let Some(object) = activity.object.as_mut_object() {
            crate::activitypub::clean_object(object);

            let object_value = serde_json::to_value(&object)?;
            crate::database::object::insert(Uuid::new_v4(), actor_db.id, object_value).await?;

            activity.object = ObjectField::Url(object.id.to_owned());
        } else if activity.object.as_actor().is_some() {
            return Err(Error::UnknownActivity);
        }

        let activity_value = serde_json::to_value(&activity)?;
        crate::database::object::insert(Uuid::new_v4(), actor_db.id, activity_value).await?;

        Ok(activity)
    } else {
        debug!(url, "Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

pub async fn fetch_actor(url: &str) -> Result<(Actor, DbActor), Error> {
    debug!(url, "Fetching remote actor...");

    match crate::database::actor::select::by_url(url).await {
        Ok(actor) => return Ok((serde_json::from_value(actor.actor.clone())?, actor)),
        Err(e) => debug!(
            url,
            error = ?e,
            "Actor not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Actor(mut actor) = fetch_entity(url).await? {
        crate::activitypub::clean_actor(&mut actor);

        let db_actor =
            crate::database::actor::insert::remote(actor.username.as_ref(), &actor).await?;

        Ok((actor, db_actor))
    } else {
        debug!(url, "Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

pub async fn fetch_object(url: &str) -> Result<Object, Error> {
    debug!(url, "Fetching remote object...");

    match crate::database::object::select::by_url(url).await {
        Ok(object) => return Ok(serde_json::from_value(object.data)?),
        Err(e) => debug!(
            url,
            error = ?e,
            "Object not found in database. Attempting remote fetch..."
        ),
    }

    if let Entity::Object(mut object) = fetch_entity(url).await? {
        crate::activitypub::clean_object(&mut object);

        let (_actor, actor_db) = fetch_actor(object.attributed_to.as_ref()).await?;

        let object_value = serde_json::to_value(&object)?;
        crate::database::object::insert(Uuid::new_v4(), actor_db.id, object_value).await?;

        Ok(object)
    } else {
        debug!(url, "Remote server returned content we can't interpret");

        Err(Error::Fetch)
    }
}

async fn fetch_entity<T: IntoUrl + Send>(url: T) -> Result<Entity, Error> {
    let client = &crate::util::REQWEST_CLIENT;
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
