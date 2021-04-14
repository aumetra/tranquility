// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use {
    crate::{
        consts::PROPER_VERSION,
        state::{ArcState, State},
    },
    argh::FromArgs,
    std::process,
    tracing_subscriber::{
        filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
        Registry,
    },
};

#[cfg(feature = "jaeger")]
use tracing_opentelemetry::OpenTelemetryLayer;

#[derive(FromArgs)]
#[argh(description = "An ActivityPub server ^_^")]
pub struct Opts {
    #[argh(option, default = "String::from(\"config.toml\")")]
    /// path to the configuration file (defaults to `config.toml`)
    config: String,

    #[argh(switch, short = 'v')]
    /// verbosity of the tracing output
    verbose: u8,

    #[argh(switch, short = 'V')]
    /// print the version
    version: bool,
}

/// Initialise the tracing subscriber
fn init_tracing(level: LevelFilter) {
    let subscriber = Registry::default()
        .with(EnvFilter::default().add_directive(level.into()))
        .with(fmt::layer());

    #[cfg(feature = "jaeger")]
    let subscriber = {
        let jaeger_tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name(env!("CARGO_PKG_NAME"))
            .install_simple()
            .unwrap();

        subscriber.with(OpenTelemetryLayer::new(jaeger_tracer))
    };

    subscriber.init();
}

/// - Initialises the tracing verbosity levels  
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

    init_tracing(level);

    let config = crate::config::load(options.config).await;
    let db_pool = crate::database::connection::init_pool(&config.server.database_url)
        .await
        .expect("Couldn't connect to database");

    State::new(config, db_pool)
}
