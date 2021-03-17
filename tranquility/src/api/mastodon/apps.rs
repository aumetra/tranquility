use {
    super::{convert::IntoMastodon, urlencoded_or_json},
    crate::state::ArcState,
    serde::Deserialize,
    uuid::Uuid,
    warp::{Filter, Rejection, Reply},
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

async fn create(state: ArcState, form: RegisterForm) -> Result<impl Reply, Rejection> {
    let client_id = Uuid::new_v4();
    let client_secret = crate::crypto::token::generate()?;

    let application = crate::database::oauth::application::insert(
        &state.db_pool,
        form.client_name,
        client_id,
        client_secret,
        form.redirect_uris,
        form.scopes,
        form.website,
    )
    .await?;
    let mastodon_application = application.into_mastodon(&state).await?;

    Ok(warp::reply::json(&mastodon_application))
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state = crate::state::filter(state);

    warp::path!("apps")
        .and(warp::post())
        .and(state)
        .and(urlencoded_or_json())
        .and_then(create)
}
