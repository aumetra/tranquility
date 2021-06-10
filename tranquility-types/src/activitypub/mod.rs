use serde_json::{json, Value};

pub const PUBLIC_IDENTIFIER: &str = "https://www.w3.org/ns/activitystreams#Public";

pub const OUTBOX_FOLLOW_COLLECTIONS_TYPE: &str = "OrderedCollection";
pub const OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE: &str = "OrderedCollectionPage";

/// Get the `@context` field value
pub fn context_field() -> Value {
    json!(["https://www.w3.org/ns/activitystreams"])
}

pub mod activity;
pub mod actor;
pub mod attachment;
pub mod collection;
pub mod object;
pub mod tag;
pub mod traits;

pub use activity::Activity;
pub use actor::{Actor, PublicKey};
pub use attachment::Attachment;
pub use collection::Collection;
pub use object::Object;
pub use tag::Tag;
pub use traits::{ApPublished, IsPrivate, IsPublic, IsUnlisted};
