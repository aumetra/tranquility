#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(clippy::doc_markdown, clippy::module_name_repetitions)]
// Needed because of conditional compilation
#![allow(clippy::used_underscore_binding)]

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
    cli::run().await;
    daemon::start();

    server::run().await;
}

mod activitypub;
mod api;
mod cli;
mod config;
mod consts;
mod crypto;
mod daemon;
mod database;

#[cfg(feature = "email")]
mod email;

mod error;
mod macros;
mod server;
mod state;
mod util;
mod well_known;

#[cfg(test)]
mod tests;
