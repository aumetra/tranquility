use {std::env, structopt::StructOpt};

#[derive(StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), version = crate::util::VERSION)]
pub struct Opts {
    #[structopt(default_value = "config.toml", long)]
    config: String,

    #[structopt(
        about = "Sets the verbosity of the tracing output",
        long,
        parse(from_occurrences),
        short
    )]
    verbose: i32,
}

pub async fn run() {
    let options = Opts::from_args();

    match options.verbose {
        0 => env::set_var("RUST_LOG", "info"),
        1 => env::set_var("RUST_LOG", "debug"),
        _ => env::set_var("RUST_LOG", "trace"),
    }

    tracing_subscriber::fmt::init();

    crate::config::init_once_cell(options.config).await;
}
