use {
    super::{convert::IntoMastodon, urlencoded_or_json},
    crate::{
        database::{InsertExt, InsertOAuthApplication},
        limit_body_size,
    },
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

async fn create(form: RegisterForm) -> Result<impl Reply, Rejection> {
    let client_id = Uuid::new_v4();
    let client_secret = crate::crypto::token::generate()?;

    let state = crate::state::get();
    let application = InsertOAuthApplication {
        client_name: form.client_name,
        client_id,
        client_secret,
        redirect_uris: form.redirect_uris,
        scopes: form.scopes,
        website: form.website,
    }
    .insert(&state.db_pool)
    .await?;
    let mastodon_application = application.into_mastodon().await?;

    Ok(warp::reply::json(&mastodon_application))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let routes = warp::path!("apps")
        .and(warp::post())
        .and(urlencoded_or_json())
        .and_then(create);
    // Restrict the body size
    limit_body_size!(routes)
}
