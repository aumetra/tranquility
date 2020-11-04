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

pub mod deliverer;
pub mod fetcher;
pub mod handler;
pub mod instantiate;
pub mod routes;
