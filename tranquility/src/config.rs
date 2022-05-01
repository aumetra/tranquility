use serde::Deserialize;
use std::path::Path;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the email configuration values
pub struct ConfigurationEmail {
    pub active: bool,

    pub server: String,
    pub starttls: bool,

    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the instance specific configuration values
pub struct ConfigurationInstance {
    pub closed_registrations: bool,
    pub domain: String,

    pub description: String,

    pub character_limit: usize,
    pub upload_limit: u64,

    pub moderators: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the jaeger specific configuration values
pub struct ConfigurationJaeger {
    pub active: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the ratelimit specific configuration values
pub struct ConfigurationRatelimit {
    pub active: bool,

    pub authentication_quota: u32,
    pub registration_quota: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the HTTP server specific configuration values
pub struct ConfigurationServer {
    pub interface: String,
    pub port: u16,

    pub database_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the TLS specific configuration values
pub struct ConfigurationTls {
    pub serve_tls_directly: bool,

    pub certificate: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Struct holding the configuration values
pub struct Configuration {
    pub email: ConfigurationEmail,
    pub instance: ConfigurationInstance,
    pub jaeger: ConfigurationJaeger,
    pub ratelimit: ConfigurationRatelimit,
    pub server: ConfigurationServer,
    pub tls: ConfigurationTls,
}

/// Load the configuration from the path
pub async fn load<P>(config_path: P) -> Configuration
where
    P: AsRef<Path>,
{
    let config_file = File::open(config_path)
        .await
        .expect("Couldn't open configuration file");
    let mut config_file = BufReader::new(config_file);

    let mut data = Vec::new();
    config_file
        .read_to_end(&mut data)
        .await
        .expect("Couldn't read configuration file");

    toml::from_slice(data.as_slice()).expect("Invalid TOML")
}
