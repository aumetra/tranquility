use serde_json::{json, Value};

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const PUBLIC_IDENTIFIER: &str = "https://www.w3.org/ns/activitystreams#Public";

pub const OUTBOX_FOLLOW_COLLECTIONS_TYPE: &str = "OrderedCollection";
pub const OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE: &str = "OrderedCollectionPage";

pub fn context_field() -> Value {
    json!(["https://www.w3.org/ns/activitystreams"])
}

macro_rules! is_public_unlisted_traits {
    ($($type:ty),*) => {
        $(
            impl IsPublic for $type {
                fn is_public(&self) -> bool {
                    self.to.contains(&PUBLIC_IDENTIFIER.to_string())
                }
            }

            impl IsUnlisted for $type {
                fn is_unlisted(&self) -> bool {
                    self.cc.contains(&PUBLIC_IDENTIFIER.to_string())
                }
            }
        )*
    }
}

pub trait IsPublic {
    fn is_public(&self) -> bool;
}

pub trait IsUnlisted {
    fn is_unlisted(&self) -> bool;
}

is_public_unlisted_traits!(Activity, Object);

pub trait IsPrivate {
    fn is_private(&self) -> bool;
}

impl<T> IsPrivate for T
where
    T: IsPublic + IsUnlisted,
{
    fn is_private(&self) -> bool {
        !self.is_public() && !self.is_unlisted()
    }
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
