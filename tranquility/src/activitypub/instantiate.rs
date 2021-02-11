use {
    super::activitypub_datetime,
    crate::format_uuid,
    tranquility_types::activitypub::{activity::ObjectField, Activity, Actor, Object, PublicKey},
    uuid::Uuid,
};

pub fn activity<T: Into<ObjectField>>(
    r#type: &str,
    owner_url: &str,
    object: T,
    to: Vec<String>,
    cc: Vec<String>,
) -> (Uuid, Activity) {
    let config = crate::config::get();

    let prefix = format!("https://{}", config.instance.domain);

    let uuid = Uuid::new_v4();
    let id = format!("{}/objects/{}", prefix, format_uuid!(uuid));

    let activity = Activity {
        id,
        r#type: r#type.into(),

        actor: owner_url.into(),

        object: object.into(),
        published: activitypub_datetime(),

        to,
        cc,

        ..Activity::default()
    };

    (uuid, activity)
}

pub fn actor(user_id: &str, username: &str, public_key_pem: String) -> Actor {
    let config = crate::config::get();

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

pub fn object(
    owner_url: &str,
    summary: &str,
    content: &str,
    sensitive: bool,
    to: Vec<String>,
    cc: Vec<String>,
) -> (Uuid, Object) {
    let config = crate::config::get();

    let prefix = format!("https://{}", config.instance.domain);

    let uuid = Uuid::new_v4();
    let id = format!("{}/objects/{}", prefix, format_uuid!(uuid));

    let object = Object {
        id,

        summary: summary.into(),
        content: content.into(),
        sensitive,
        published: activitypub_datetime(),

        attributed_to: owner_url.into(),

        to,
        cc,

        ..Object::default()
    };

    (uuid, object)
}
