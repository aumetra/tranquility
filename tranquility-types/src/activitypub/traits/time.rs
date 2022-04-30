use crate::activitypub::{Activity, Object};
use chrono::{DateTime, FixedOffset, ParseResult};

/// Implement the macro for ActivityPub entities that have a `published` field
macro_rules! impl_activitypub_published {
    ($($entity:ty),+) => {
        $(
            impl ApPublished for $entity {
                fn published_parsed(&self) -> chrono::ParseResult<chrono::DateTime<chrono::FixedOffset>> {
                    DateTime::parse_from_rfc3339(&self.published)
                }
            }
        )+
    }
}

/// Trait for parsing the `published` timestamp
pub trait ApPublished {
    /// Get the `published` field from ActivityPub entities in a parsed chrono format
    fn published_parsed(&self) -> ParseResult<DateTime<FixedOffset>>;
}

impl_activitypub_published!(Activity, Object);
