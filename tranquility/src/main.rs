#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate tracing;

use {once_cell::sync::Lazy, reqwest::Client, std::env};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static REQWEST_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().user_agent(USER_AGENT).build().unwrap());

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");

    cli::run().await;
}

mod activitypub;
mod api;
mod cli;
mod config;
mod crypto;
mod database;
mod deliverer;
mod error;
mod fetcher;
mod server;

#[cfg(test)]
mod tests;
