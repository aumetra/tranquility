use {
    super::{convert::IntoMastodon, urlencoded_or_json},
    crate::{
        database::{InsertExt, InsertOAuthApplication},
        state::ArcState,
        unrejectable_err,
    },
    serde::Deserialize,
    uuid::Uuid,
    warp::{reply::Response, Filter, Rejection, Reply},
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

async fn create(state: ArcState, form: RegisterForm) -> Result<Response, Rejection> {
    let client_id = Uuid::new_v4();
    let client_secret = unrejectable_err!(crate::crypto::token::generate());

    let application = unrejectable_err!(
        InsertOAuthApplication {
            client_name: form.client_name,
            client_id,
            client_secret,
            redirect_uris: form.redirect_uris,
            scopes: form.scopes,
            website: form.website,
        }
        .insert(&state.db_pool)
        .await
    );
    let mastodon_application = unrejectable_err!(application.into_mastodon(&state).await);

    Ok(warp::reply::json(&mastodon_application).into_response())
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state = crate::state::filter(state);

    warp::path!("apps")
        .and(warp::post())
        .and(state)
        .and(urlencoded_or_json())
        .and_then(create)
}
