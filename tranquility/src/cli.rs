use {
    crate::{config::ArcConfig, consts::PROPER_VERSION},
    std::sync::Arc,
    structopt::StructOpt,
    tracing_subscriber::filter::LevelFilter,
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

/// - Initializes the tracing verbosity levels  
/// - Initializes the database connection pool  
/// - Returns the loaded configuration inside an arc
pub async fn run() -> ArcConfig {
    let options = Opts::from_args();

    let level = match options.verbose {
        0 => LevelFilter::INFO,
        1 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    let config = crate::config::load(options.config).await;
    crate::database::connection::init_pool(&config.server.database_url).await;

    Arc::new(config)
}
