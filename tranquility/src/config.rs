use {
    once_cell::sync::OnceCell,
    serde::Deserialize,
    std::{fs::File, io::BufReader},
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

pub fn init_once_cell(config_path: String) {
    let config_file = File::open(config_path).unwrap();
    let config_file = BufReader::new(config_file);

    CONFIGURATION
        .set(serde_json::from_reader(config_file).unwrap())
        .ok();
}

pub fn get() -> &'static Configuration {
    CONFIGURATION.get().unwrap()
}
