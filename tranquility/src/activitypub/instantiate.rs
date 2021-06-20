#![allow(clippy::too_many_arguments)]

use {
    super::current_datetime,
    crate::{config::Configuration, format_uuid},
    tranquility_types::activitypub::{
        activity::ObjectField, Activity, Actor, Object, PublicKey, Tag,
    },
    uuid::Uuid,
};

/// Instantiate an ActivityPub activity
pub fn activity<T: Into<ObjectField>>(
    config: &Configuration,
    r#type: &str,
    owner_url: &str,
    object: T,
    to: Vec<String>,
    cc: Vec<String>,
) -> (Uuid, Activity) {
    let prefix = format!("https://{}", config.instance.domain);

    let uuid = Uuid::new_v4();
    let id = format!("{}/objects/{}", prefix, format_uuid!(uuid));

    let activity = Activity {
        id,
        r#type: r#type.into(),

        actor: owner_url.into(),

        object: object.into(),
        published: current_datetime(),

        to,
        cc,

        ..Activity::default()
    };

    (uuid, activity)
}

/// Instantiate an ActivityPub actor
pub fn actor(
    config: &Configuration,
    user_id: &str,
    username: &str,
    public_key_pem: String,
) -> Actor {
    let prefix = format!("https://{}", config.instance.domain);
    let id = format!("{}/users/{}", prefix, user_id);

    let inbox = format!("{}/inbox", id);
    let outbox = format!("{}/outbox", id);

    let followers = format!("{}/followers", id);
    let following = format!("{}/following", id);

    let key_id = format!("{}#main-key", id);

    let public_key = PublicKey {
        id: key_id,
        owner: id.clone(),
        public_key_pem,
    };

    Actor {
        id,
        r#type: "Person".into(),

        username: username.into(),

        inbox,
        outbox,

        followers,
        following,

        public_key,

        ..Actor::default()
    }
}

/// Instantiate an ActivityPub object
pub fn object(
    config: &Configuration,
    owner_url: &str,
    summary: &str,
    content: &str,
    tags: Option<Vec<Tag>>,
    sensitive: bool,
    to: Vec<String>,
    cc: Vec<String>,
) -> (Uuid, Object) {
    let prefix = format!("https://{}", config.instance.domain);

    let uuid = Uuid::new_v4();
    let id = format!("{}/objects/{}", prefix, format_uuid!(uuid));

    let object = Object {
        id,

        summary: summary.into(),
        content: content.into(),
        tag: tags.unwrap_or_default(),
        sensitive,
        published: current_datetime(),

        attributed_to: owner_url.into(),

        to,
        cc,

        ..Object::default()
    };

    (uuid, object)
}
