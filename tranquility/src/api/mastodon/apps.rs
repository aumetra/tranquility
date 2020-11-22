use {
    super::convert::IntoMastodon,
    serde::Deserialize,
    uuid::Uuid,
    warp::{Rejection, Reply},
};

fn default_scopes() -> String {
    "read".into()
}

#[derive(Deserialize)]
pub struct RegisterForm {
    client_name: String,
    redirect_uris: String,
    #[serde(default = "default_scopes")]
    scopes: String,
    #[serde(default)]
    website: String,
}

pub async fn create(form: RegisterForm) -> Result<impl Reply, Rejection> {
    let client_id = Uuid::new_v4();
    let client_secret = crate::crypto::token::generate()?;

    let application = crate::database::oauth::application::insert(
        form.client_name,
        client_id,
        client_secret,
        form.redirect_uris,
        form.scopes,
        form.website,
    )
    .await?;
    let mastodon_application = application.into_mastodon().await?;

    Ok(warp::reply::json(&mastodon_application))
}
