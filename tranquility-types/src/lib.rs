#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::doc_markdown,
    clippy::struct_excessive_bools,
    clippy::missing_errors_doc,
    clippy::must_use_candidate
)]

#[cfg(feature = "activitypub")]
pub mod activitypub;
#[cfg(feature = "mastodon")]
pub mod mastodon;
#[cfg(feature = "nodeinfo")]
pub mod nodeinfo;
#[cfg(feature = "webfinger")]
pub mod webfinger;

#[cfg(test)]
mod tests;

mod util;
