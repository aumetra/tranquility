use {
    once_cell::sync::OnceCell,
    serde::Deserialize,
    tokio::{
        fs::File,
        io::{AsyncReadExt, BufReader},
    },
};

static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigurationInstance {
    pub closed_registrations: bool,
    pub domain: String,

    pub character_limit: usize,
    pub upload_limit: usize,

    pub moderators: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigurationRatelimit {
    pub active: bool,

    pub authentication_quota: u32,
    pub registration_quota: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigurationServer {
    pub port: u16,

    pub database_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigurationTls {
    pub serve_tls_directly: bool,

    pub certificate: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    pub instance: ConfigurationInstance,
    pub ratelimit: ConfigurationRatelimit,
    pub server: ConfigurationServer,
    pub tls: ConfigurationTls,
}

pub async fn init_once_cell(config_path: String) {
    let config_file = File::open(config_path)
        .await
        .expect("Couldn't open configuration file");
    let mut config_file = BufReader::new(config_file);

    let mut data = Vec::new();
    config_file
        .read_to_end(&mut data)
        .await
        .expect("Couldn't read configuration file");
    CONFIGURATION
        .set(toml::from_slice(data.as_slice()).expect("Invalid TOML"))
        .ok()
        .expect("OnceCell already initialized");
}

pub fn get() -> &'static Configuration {
    CONFIGURATION.get().unwrap()
}
