// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use crate::{
    config::Configuration,
    consts::PROPER_VERSION,
    state::{ArcState, State},
};
use argh::FromArgs;
use std::process;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

#[cfg(feature = "jaeger")]
use tracing_opentelemetry::OpenTelemetryLayer;

#[derive(FromArgs)]
#[argh(description = "An ActivityPub server ^_^")]
pub struct Opts {
    #[argh(option, default = "\"config.toml\".into()")]
    /// path to the configuration file (defaults to `config.toml`)
    config: String,

    #[argh(switch, short = 'v')]
    /// print the version
    version: bool,
}

/// Initialise the tracing subscriber
fn init_tracing(_config: &Configuration) {
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer());

    #[cfg(feature = "jaeger")]
    {
        if _config.jaeger.active {
            let host = _config.jaeger.host.as_str();
            let port = _config.jaeger.port;

            let jaeger_endpoint = opentelemetry_jaeger::new_pipeline()
                .with_service_name(env!("CARGO_PKG_NAME"))
                .with_agent_endpoint((host, port))
                .install_batch(opentelemetry::runtime::Tokio)
                .expect("Couldn't install jaeger pipeline");

            subscriber
                .with(OpenTelemetryLayer::new(jaeger_endpoint))
                .init();
            return;
        }
    }

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

    let config = crate::config::load(options.config).await;
    init_tracing(&config);

    let db_pool = crate::database::connection::init_pool(&config.server.database_url)
        .await
        .expect("Couldn't connect to database");

    State::new(config, db_pool)
}
