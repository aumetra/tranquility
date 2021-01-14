use {clap::Clap, std::env};

#[derive(Clap)]
#[clap(version = crate::VERSION, author = env!("CARGO_PKG_AUTHORS"))]
pub struct Opts {
    #[clap(long, default_value = "config.json")]
    config: String,
    #[clap(
        short,
        long,
        about = "Sets the verbosity of the tracing output",
        parse(from_occurrences)
    )]
    verbose: i32,
}

pub async fn run() {
    let options = Opts::parse();

    match options.verbose {
        0 => env::set_var("RUST_LOG", "info"),
        1 => env::set_var("RUST_LOG", "debug"),
        _ => env::set_var("RUST_LOG", "trace"),
    }

    tracing_subscriber::fmt::init();

    crate::config::init_once_cell(options.config).await;
}
