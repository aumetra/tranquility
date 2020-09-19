use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");

    cli::run().await;
}

mod cli;
mod config;
mod error;
mod fetcher;
mod hashing;
mod routes;
mod server;
