// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use {
    crate::{config::ArcConfig, consts::PROPER_VERSION},
    argh::FromArgs,
    std::{process, sync::Arc},
    tracing_subscriber::filter::LevelFilter,
};

#[derive(FromArgs)]
#[argh(description = "An ActivityPub server ^_^")]
pub struct Opts {
    #[argh(option, default = "String::from(\"config.toml\")")]
    /// path to the configuration file (defaults to `config.toml`)
    config: String,

    #[argh(switch, short = 'v')]
    /// set the verbosity of the tracing output
    verbose: u8,

    #[argh(switch, short = 'V')]
    /// print the version
    version: bool,
}

/// - Initializes the tracing verbosity levels  
/// - Initializes the database connection pool  
/// - Returns the loaded configuration inside an arc
pub async fn run() -> ArcConfig {
    let options = argh::from_env::<Opts>();

    if options.version {
        println!("{}", PROPER_VERSION);
        process::exit(0);
    }

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
