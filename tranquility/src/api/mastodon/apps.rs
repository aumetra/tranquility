use {
    serde::Deserialize,
    warp::{Rejection, Reply},
};

fn default_scopes() -> String {
    "read".into()
}

#[derive(Deserialize)]
pub struct RegisterForm {
    client_id: String,
    redirect_uris: String,
    #[serde(default = "default_scopes")]
    scopes: String,
    #[serde(default)]
    website: String,
}

pub async fn create(form: RegisterForm) -> Result<impl Reply, Rejection> {
    Ok("client id and client secret")
}
