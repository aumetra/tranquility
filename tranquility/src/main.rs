#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate tracing;

use {once_cell::sync::Lazy, reqwest::Client, std::env};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("GIT_BRANCH"),
    "-",
    env!("GIT_COMMIT")
);

static REQWEST_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().user_agent(USER_AGENT).build().unwrap());

#[tokio::main]
async fn main() {
    cli::run();

    crate::database::init().await.unwrap();
    crate::daemon::start();
    crate::server::run().await;
}

mod activitypub;
mod api;
mod cli;
mod config;
mod crypto;
mod daemon;
mod database;
mod error;
mod server;
mod util;
mod webfinger;

#[cfg(test)]
mod tests;
