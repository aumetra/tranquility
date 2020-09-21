use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");

    cli::run().await;
}

mod cli;
mod config;
mod crypto;
mod database;
mod error;
mod fetcher;
mod routes;
mod server;
