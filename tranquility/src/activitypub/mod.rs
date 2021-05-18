use {
    serde::{Deserialize, Serialize},
    tranquility_types::activitypub::{
        Activity, Actor, IsPrivate, IsUnlisted, Object, DATE_TIME_FORMAT,
    },
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

impl IsUnlisted for ActivityObject {
    fn is_unlisted(&self) -> bool {
        match self {
            ActivityObject::Activity(activity) => activity.is_unlisted(),
            ActivityObject::Object(object) => object.is_unlisted(),
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

/// Extension trait for cleaning objects from potentially malicious HTML
pub trait Clean {
    /// Clean any fields that could potentially contain malicious HTML
    fn clean(&mut self);
}

impl Clean for Actor {
    fn clean(&mut self) {
        self.name = ammonia::clean(self.name.as_str());
        self.summary = ammonia::clean(self.summary.as_str());
    }
}

impl Clean for Object {
    fn clean(&mut self) {
        self.summary = ammonia::clean(self.summary.as_str());
        self.content = ammonia::clean(self.content.as_str());
    }
}

pub mod deliverer;
pub mod fetcher;
pub mod handler;
pub mod instantiate;
pub mod interactions;
pub mod routes;
