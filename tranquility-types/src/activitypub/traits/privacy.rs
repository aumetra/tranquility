use crate::{
    activitypub::{Activity, Object, PUBLIC_IDENTIFIER},
    util::contains,
};

/// Macro that implements the [IsPublic] and [IsUnlisted] trait for an struct
///
/// The struct has to have `to` and `cc` fields that can be implicitly converted to an `&[String]` when referenced
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

/// Trait to check whether the ActivityPub entity is public
pub trait IsPublic {
    /// Everyone is allowed to see the post and should appear in every timeline  
    fn is_public(&self) -> bool;
}

/// Trait to check whether the ActivityPub entity is unlisted
pub trait IsUnlisted {
    /// Everyone is allowed to see the post but it should only appear in the follower's home timelines  
    fn is_unlisted(&self) -> bool;
}

is_public_unlisted_traits!(Activity, Object);

/// Trait to check whether the ActivityPub entity is private
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
