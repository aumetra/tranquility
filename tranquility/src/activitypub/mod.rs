use {
    serde::{Deserialize, Serialize},
    tranquility_types::activitypub::{activity::Activity, Object, DATE_TIME_FORMAT},
};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct FollowActivity {
    #[serde(flatten)]
    pub activity: Activity,
    #[serde(default)]
    pub approved: bool,
}

fn activitypub_datetime() -> String {
    chrono::Utc::now().format(DATE_TIME_FORMAT).to_string()
}

pub fn clean_object(object: &mut Object) {
    object.content = ammonia::clean(&object.content);
}

pub mod create {
    use {
        super::activitypub_datetime,
        tranquility_types::activitypub::{
            activity::ObjectField, Activity, Actor, Object, PublicKey,
        },
    };

    pub fn activity<T: Into<ObjectField>>(
        r#type: &str,
        id: &str,
        owner_url: &str,
        object: T,
        to: Vec<String>,
        cc: Vec<String>,
    ) -> Activity {
        let config = crate::config::get();

        let prefix = format!("https://{}", config.domain);
        let id = format!("{}/objects/{}", prefix, id);

        Activity {
            id,
            r#type: r#type.into(),

            actor: owner_url.into(),

            object: object.into(),
            published: activitypub_datetime(),

            to,
            cc,
            ..Activity::default()
        }
    }

    pub fn actor(user_id: &str, username: &str, public_key_pem: String) -> Actor {
        let config = crate::config::get();

        let prefix = format!("https://{}", config.domain);
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
        id: &str,
        owner_url: &str,
        content: &str,
        to: Vec<String>,
        cc: Vec<String>,
    ) -> Object {
        let config = crate::config::get();

        let prefix = format!("https://{}", config.domain);
        let id = format!("{}/objects/{}", prefix, id);

        Object {
            id,

            content: content.into(),
            published: activitypub_datetime(),

            attributed_to: owner_url.into(),

            to,
            cc,
            ..Object::default()
        }
    }
}

pub mod deliverer;
pub mod fetcher;
pub mod handler;
pub mod routes;
