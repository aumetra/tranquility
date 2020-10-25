use {
    serde::{Deserialize, Serialize},
    tranquility_types::activitypub::{
        activity::{Activity, ObjectField},
        Actor, PublicKey,
    },
};

fn default_approved() -> bool {
    true
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FollowActivity {
    #[serde(flatten)]
    pub activity: Activity,
    #[serde(default = "default_approved")]
    pub approved: bool,
}

pub fn create_activity<T: Into<ObjectField>>(
    r#type: &str,
    id: &str,
    owner_url: &str,
    object: T,
    to: Vec<String>,
    cc: Vec<String>,
) -> Activity {
    let config = crate::config::get();

    let prefix = format!("https://{}", config.domain);
    let id = format!("{}/activity/{}", prefix, id);

    Activity {
        id,
        r#type: r#type.into(),

        actor: owner_url.into(),

        object: object.into(),

        to,
        cc,
        ..Activity::default()
    }
}

pub fn create_actor(user_id: &str, username: &str, public_key_pem: String) -> Actor {
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

pub mod handler;
pub mod routes;
