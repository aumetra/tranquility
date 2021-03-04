use {
    crate::consts::PROPER_VERSION, structopt::StructOpt, tracing_subscriber::filter::LevelFilter,
};

#[derive(StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), version = PROPER_VERSION)]
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

    let level = match options.verbose {
        0 => LevelFilter::INFO,
        1 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    crate::config::init_once_cell(options.config).await;
}
