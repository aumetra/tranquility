use super::convert::IntoMastodon;
use crate::{
    database::{InsertExt, InsertOAuthApplication},
    error::Error,
    state::ArcState,
};
use axum::{extract::Form, response::IntoResponse, routing::post, Extension, Json, Router};
use serde::Deserialize;
use uuid::Uuid;

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

async fn create(
    Extension(state): Extension<ArcState>,
    Form(form): Form<RegisterForm>,
) -> Result<impl IntoResponse, Error> {
    let client_id = Uuid::new_v4();
    let client_secret = crate::crypto::token::generate();

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
    let mastodon_application = application.into_mastodon(&state).await?;

    Ok(Json(&mastodon_application))
}

pub fn routes() -> Router {
    Router::new().route("/apps", post(create))
}
