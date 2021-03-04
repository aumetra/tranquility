#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

#[cfg(all(feature = "jemalloc", not(feature = "mimalloc"), not(test)))]
#[global_allocator]
static GLOBAL: jemalloc::Jemalloc = jemalloc::Jemalloc;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc"), not(test)))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    let config = cli::run().await;

    database::migrate()
        .await
        .expect("Database migration failed");
    daemon::start();
    server::run(config).await;
}

mod activitypub;
mod api;
mod cli;
mod config;
mod consts;
mod crypto;
mod daemon;
mod database;
mod error;
mod server;
mod util;
mod webfinger;

#[cfg(test)]
mod tests;
