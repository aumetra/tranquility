// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use {
    crate::{
        config::Configuration,
        consts::PROPER_VERSION,
        state::{ArcState, State},
    },
    argh::FromArgs,
    cfg_if::cfg_if,
    std::process,
    tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry},
};

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

    cfg_if! {
        if #[cfg(feature = "jaeger")] {
            let host = _config.jaeger.host.as_str();
            let port = _config.jaeger.port;

            let jaeger_endpoint = opentelemetry_jaeger::new_pipeline()
                .with_service_name(env!("CARGO_PKG_NAME"))
                .with_agent_endpoint((host, port))
                .install_batch(opentelemetry::runtime::Tokio);

            // Try to connect to the jaeger endpoint
            // If it works, great. If not, log and move on
            match jaeger_endpoint {
                Ok(endpoint) => subscriber.with(OpenTelemetryLayer::new(endpoint)).init(),
                Err(err) => warn!(error = ?err, "Jaeger exporter couldn't be initialised")
            }
        } else {
            subscriber.init();
        }
    }
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
    let db_pool = crate::database::connection::init_pool(&config.server.database_url)
        .await
        .expect("Couldn't connect to database");

    init_tracing(&config);

    State::new(config, db_pool)
}
