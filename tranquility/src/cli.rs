use clap::Clap;
use std::env;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Opts {
    #[clap(long, default_value = "config.json")]
    config: String,
    #[clap(short, long, about = "Sets the logger to the logging level \"DEBUG\"")]
    verbose: bool,
}

pub async fn run() {
    let options = Opts::parse();

    if options.verbose {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    crate::config::init_once_cell(options.config);
    crate::server::run().await;
}
