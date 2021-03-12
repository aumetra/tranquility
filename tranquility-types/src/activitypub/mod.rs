use serde_json::{json, Value};

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const PUBLIC_IDENTIFIER: &str = "https://www.w3.org/ns/activitystreams#Public";

pub const OUTBOX_FOLLOW_COLLECTIONS_TYPE: &str = "OrderedCollection";
pub const OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE: &str = "OrderedCollectionPage";

pub fn context_field() -> Value {
    json!(["https://www.w3.org/ns/activitystreams"])
}

#[inline]
/// A replacement for `<array>.contains(<value>)` because `.contains()` because, for example, the `.contains()` of `Vec<String>` can't be used with an `&str`  
fn contains(vec: &[String], value: &str) -> bool {
    vec.iter().any(|entry| entry == value)
}

macro_rules! is_public_unlisted_traits {
    ($($type:ty),+) => {
        $(
            impl IsPublic for $type {
                fn is_public(&self) -> bool {
                    contains(&self.to, PUBLIC_IDENTIFIER)
                }
            }

            impl IsUnlisted for $type {
                fn is_unlisted(&self) -> bool {
                    contains(&self.cc, PUBLIC_IDENTIFIER)
                }
            }
        )+
    }
}

pub trait IsPublic {
    /// Everyone is allowed to see the post and should appear in every timeline  
    fn is_public(&self) -> bool;
}

pub trait IsUnlisted {
    /// Everyone is allowed to see the post but it should only appear in the follower's home timelines  
    fn is_unlisted(&self) -> bool;
}

is_public_unlisted_traits!(Activity, Object);

pub trait IsPrivate {
    /// Only followers and/or mentioned users are allowed to see the post and it should only appear in the aforementioned user groups home timelines  
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
