use serde_json::{json, Value};

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const PUBLIC_IDENTIFIER: &str = "https://www.w3.org/ns/activitystreams#Public";

pub fn context_field() -> Value {
    json!(["https://www.w3.org/ns/activitystreams"])
}

pub mod activity;
pub mod actor;
pub mod attachment;
pub mod collection;
pub mod object;
pub mod tag;

pub use activity::Activity;
pub use actor::{Actor, PublicKey};
pub use attachment::Attachment;
pub use collection::Collection;
pub use object::Object;
pub use tag::Tag;
