// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use {
    crate::{
        consts::PROPER_VERSION,
        state::{ArcState, State},
    },
    argh::FromArgs,
    std::process,
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
/// - Creates a database connection pool  
/// - Returns a constructed state  
pub async fn run() -> ArcState {
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
    let db_pool = crate::database::connection::init_pool(&config.server.database_url)
        .await
        .expect("Couldn't connect to database");

    State::new(config, db_pool)
}
