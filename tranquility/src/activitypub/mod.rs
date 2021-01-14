use {
    serde::{Deserialize, Serialize},
    tranquility_types::activitypub::{activity::Activity, IsPrivate, Object, DATE_TIME_FORMAT},
};

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum ActivityObject {
    Activity(Activity),
    Object(Object),
}

impl IsPrivate for ActivityObject {
    fn is_private(&self) -> bool {
        match self {
            ActivityObject::Activity(activity) => activity.is_private(),
            ActivityObject::Object(object) => object.is_private(),
        }
    }
}

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

pub mod deliverer;
pub mod fetcher;
pub mod handler;
pub mod instantiate;
pub mod interactions;
pub mod routes;
