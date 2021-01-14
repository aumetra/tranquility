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
#[serde(rename_all = "camelCase")]
pub struct ConfigurationTls {
    pub reverse_proxy: bool,

    pub certificate: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub port: u16,
    pub database_url: String,
    pub domain: String,
    pub tls: ConfigurationTls,
}

pub async fn init_once_cell(config_path: String) {
    let config_file = File::open(config_path)
        .await
        .expect("Couldn't open configuration file");
    let mut config_file = BufReader::new(config_file);

    let mut data = Vec::new();
    config_file.read_to_end(&mut data).await.unwrap();
    CONFIGURATION
        .set(serde_json::from_slice(data.as_slice()).expect("Invalid JSON"))
        .ok()
        .expect("OnceCell already initialized");
}

pub fn get() -> &'static Configuration {
    CONFIGURATION.get().unwrap()
}
