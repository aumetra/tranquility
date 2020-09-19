use crate::error::Error;
use reqwest::IntoUrl;
use serde_json::Value;
use tranquility_types::activitypub::{Activity, Actor, Object};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub enum Entity {
    Activity(Activity),
    Actor(Actor),
    Object(Object),
    Other,
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
        // This should be deserializable into an object
        let object = serde_json::from_value(entity)?;

        Entity::Object(object)
    };

    Ok(entity)
}
