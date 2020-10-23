use {
    crate::error::Error,
    reqwest::IntoUrl,
    serde_json::Value,
    tranquility_types::activitypub::{Activity, Actor, Object},
};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub enum Entity {
    Activity(Activity),
    Actor(Actor),
    Object(Object),
}

impl Entity {
    pub fn into_activity(self) -> Option<Activity> {
        match self {
            Entity::Activity(activity) => Some(activity),
            _ => None,
        }
    }

    pub fn into_actor(self) -> Option<Actor> {
        match self {
            Entity::Actor(actor) => Some(actor),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<Object> {
        match self {
            Entity::Object(object) => Some(object),
            _ => None,
        }
    }
}

pub async fn fetch_activity(url: String) -> Result<Activity, Error> {
    match crate::database::activity::select::by_url(url.clone()).await {
        Ok(activity) => return Ok(serde_json::from_value(activity.data)?),
        Err(e) => {
            debug!("{}", e);
            debug!("Activity not found in database. Attempting remote fetch...");
        }
    }

    match fetch_entity(url.as_str()).await? {
        Entity::Activity(activity) => {
            let actor = fetch_actor(activity.actor.clone()).await?;
            let actor = crate::database::actor::select::by_url(actor.id).await?;

            crate::database::activity::insert(actor.id, activity.clone()).await?;

            Ok(activity)
        }
        _ => {
            debug!("Remote server returned content we can't interpret");

            Err(Error::FetchError)
        }
    }
}

pub async fn fetch_actor(url: String) -> Result<Actor, Error> {
    match crate::database::actor::select::by_url(url.clone()).await {
        Ok(actor) => return Ok(serde_json::from_value(actor.actor)?),
        Err(e) => {
            debug!("{}", e);
            debug!("Actor not found in database. Attempting remote fetch...");
        }
    }

    match fetch_entity(url.as_str()).await? {
        Entity::Actor(actor) => {
            let actor_value = serde_json::to_value(&actor)?;
            crate::database::actor::insert::remote(actor.username.clone(), actor_value).await?;

            Ok(actor)
        }
        _ => {
            debug!("Remote server returned content we can't interpret");

            Err(Error::FetchError)
        }
    }
}

pub async fn fetch_entity<T: IntoUrl>(url: T) -> Result<Entity, Error> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
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
