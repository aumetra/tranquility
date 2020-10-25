#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod activitypub;
pub mod webfinger;

#[cfg(test)]
mod tests;
