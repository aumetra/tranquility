use crate::error::Error;
use reqwest::IntoUrl;
use serde_json::Value;
use tranquility_types::activitypub::{Activity, Actor, Object};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub enum Entity {
    Activity(Activity),
    Actor(Actor),
    Object(Object),
}

pub async fn fetch_actor(url: String) -> Result<Actor, Error> {
    match crate::database::actor::select::by_url(url.clone()).await {
        Ok(actor) => return Ok(serde_json::from_value(actor.actor)?),
        Err(e) => {
            log::debug!("{}", e);
            log::debug!("Actor not found in database. Attempting remote fetch...");
        }
    }

    match fetch_entity(url.as_str()).await? {
        Entity::Actor(actor) => {
            let actor_value = serde_json::to_value(&actor)?;
            crate::database::actor::insert::external(actor.username.clone(), actor_value).await?;

            Ok(actor)
        }
        _ => {
            log::debug!("Remote server returned content we can't interpret");

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
