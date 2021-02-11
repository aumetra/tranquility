#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::struct_excessive_bools, clippy::must_use_candidate)]

#[cfg(feature = "activitypub")]
pub mod activitypub;
#[cfg(feature = "mastodon")]
pub mod mastodon;
#[cfg(feature = "webfinger")]
pub mod webfinger;

#[cfg(test)]
mod tests;
