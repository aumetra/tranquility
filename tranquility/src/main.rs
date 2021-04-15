#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate tracing;

use std::sync::Arc;

#[cfg(all(feature = "jemalloc", not(feature = "mimalloc"), not(test)))]
#[global_allocator]
static GLOBAL: jemalloc::Jemalloc = jemalloc::Jemalloc;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc"), not(test)))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    let state = cli::run().await;

    database::migrate(&state.db_pool)
        .await
        .expect("Database migration failed");

    {
        let state = Arc::clone(&state);
        daemon::start(state);
    }

    server::run(state).await;
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
mod macros;
mod server;
mod state;
mod util;
mod webfinger;

#[cfg(test)]
mod tests;
