#[macro_use]
extern crate tracing;

use std::env;

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
mod error;
mod fetcher;
mod server;
