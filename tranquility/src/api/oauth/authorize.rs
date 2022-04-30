use super::{TokenTemplate, AUTHORIZE_FORM};
use crate::{
    crypto::password,
    database::{Actor, InsertExt, InsertOAuthAuthorization, OAuthApplication},
    error::Error,
    state::ArcState,
    util::Form,
};
use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

static AUTHORIZATION_CODE_VALIDITY: Duration = Duration::minutes(5);

#[derive(Deserialize)]
pub struct AuthoriseForm {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    response_type: String,
    client_id: Uuid,
    redirect_uri: String,
    // scope: Option<String>,
    #[serde(default)]
    state: String,
}

#[allow(clippy::unused_async)]
pub async fn get() -> impl IntoResponse {
    Html(AUTHORIZE_FORM.as_str())
}

#[debug_handler]
pub async fn post(
    Extension(state): Extension<ArcState>,
    Form(form): Form<AuthoriseForm>,
    Query(query): Query<QueryParams>,
) -> Result<impl IntoResponse, Error> {
    let actor = Actor::by_username_local(&state.db_pool, &form.username).await?;
    if !password::verify(form.password, actor.password_hash.unwrap()).await {
        return Err(Error::Unauthorized);
    }

    // RFC 6749:
    // ```
    // response_type
    //    REQUIRED.  Value MUST be set to "code".
    // ```
    if query.response_type != "code" {
        return Err(Error::InvalidRequest);
    }

    let client = OAuthApplication::by_client_id(&state.db_pool, &query.client_id).await?;
    if client.redirect_uris != query.redirect_uri {
        return Err(Error::InvalidRequest);
    }

    let authorization_code = crate::crypto::token::generate();
    let valid_until = OffsetDateTime::now_utc() + AUTHORIZATION_CODE_VALIDITY;

    let authorization_code = InsertOAuthAuthorization {
        application_id: client.id,
        actor_id: actor.id,
        code: authorization_code,
        valid_until,
    }
    .insert(&state.db_pool)
    .await?;

    // Display the code to the user if the redirect URI is "urn:ietf:wg:oauth:2.0:oob"
    if query.redirect_uri == "urn:ietf:wg:oauth:2.0:oob" {
        let page = TokenTemplate {
            token: authorization_code.code,
        }
        .render()?;

        Ok(Html(page).into_response())
    } else {
        let redirect_uri = format!(
            "{}?code={}&state={}",
            query.redirect_uri, authorization_code.code, query.state,
        );

        Ok(Redirect::temporary(&redirect_uri).into_response())
    }
}
