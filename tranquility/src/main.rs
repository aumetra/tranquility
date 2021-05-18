#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::doc_markdown, clippy::module_name_repetitions)]

#[macro_use]
extern crate tracing;

cfg_if::cfg_if! {
    if #[cfg(feature = "jemalloc")] {
        #[global_allocator]
        static GLOBAL: jemalloc::Jemalloc = jemalloc::Jemalloc;
    } else if #[cfg(feature = "mimalloc")] {
        #[global_allocator]
        static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
    }
}

#[tokio::main]
async fn main() {
    let state = cli::run().await;

    database::migrate(&state.db_pool)
        .await
        .expect("Database migration failed");
    daemon::start(&state);

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
