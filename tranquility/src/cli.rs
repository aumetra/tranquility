// Allow this because of expanded code of `argh`
#![allow(clippy::default_trait_access)]

use {
    crate::consts::PROPER_VERSION,
    argh::FromArgs,
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
fn init_tracing() {
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer());

    #[cfg(feature = "jaeger")]
    {
        let state = crate::state::get();
        let config = &state.config;

        if config.jaeger.active {
            let host = config.jaeger.host.as_str();
            let port = config.jaeger.port;

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
pub async fn run() {
    let options = argh::from_env::<Opts>();

    if options.version {
        println!("{}", PROPER_VERSION);
        process::exit(0);
    }

    crate::state::init(options.config).await;
    init_tracing();
}
