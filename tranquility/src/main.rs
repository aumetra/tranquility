#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate tracing;

#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(feature = "mimalloc", not(feature = "jemallocator")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    cli::run().await;

    crate::database::init()
        .await
        .expect("Database connection/migration failed");
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
