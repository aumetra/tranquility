use super::TokenTemplate;
use crate::{
    crypto::password,
    database::{Actor, InsertExt, InsertOAuthToken, OAuthApplication, OAuthAuthorization},
    error::Error,
    state::ArcState,
    util::Form,
};
use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension, Json,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

static ACCESS_TOKEN_VALID_DURATION: Duration = Duration::hours(1);

/// Form for password grant authorisation flows
#[derive(Deserialize)]
struct FormPasswordGrant {
    username: String,
    password: String,
}

/// Form for code grant authorisation flows
#[derive(Deserialize)]
struct FormCodeGrant {
    client_id: Uuid,
    client_secret: String,
    redirect_uri: String,
    // scope: Option<String>,
    code: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
enum FormData {
    CodeGrant(FormCodeGrant),
    PasswordGrant(FormPasswordGrant),
}

#[derive(Deserialize)]
pub struct TokenForm {
    grant_type: String,

    #[serde(flatten)]
    data: FormData,
}

impl FormData {
    /// If the form is a code grant form, return it otherwise return a rejection
    pub fn code_grant(self) -> Result<FormCodeGrant, Error> {
        match self {
            Self::CodeGrant(form) => Ok(form),
            _ => Err(Error::InvalidRequest),
        }
    }

    /// If the form is a password grant form, return it otherwise return a rejection
    pub fn password_grant(self) -> Result<FormPasswordGrant, Error> {
        match self {
            Self::PasswordGrant(form) => Ok(form),
            _ => Err(Error::InvalidRequest),
        }
    }
}

/// Serialisable struct for responding to an access token request
#[derive(Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: i64,
}

impl Default for AccessTokenResponse {
    fn default() -> Self {
        Self {
            token_type: "Bearer".into(),
            scope: "read write follow push".into(),

            access_token: String::new(),
            created_at: 0,
        }
    }
}

async fn code_grant(
    state: &ArcState,
    FormCodeGrant {
        client_id,
        client_secret,
        redirect_uri,
        code,
        ..
    }: FormCodeGrant,
) -> Result<impl IntoResponse, Error> {
    let client = OAuthApplication::by_client_id(&state.db_pool, &client_id).await?;
    if client.client_secret != client_secret || client.redirect_uris != redirect_uri {
        return Err(Error::Unauthorized);
    }

    let authorization_code = OAuthAuthorization::by_code(&state.db_pool, &code).await?;
    let valid_until = OffsetDateTime::now_utc() + ACCESS_TOKEN_VALID_DURATION;
    let access_token = crate::crypto::token::generate();

    let access_token = InsertOAuthToken {
        application_id: Some(client.id),
        actor_id: authorization_code.actor_id,
        access_token,
        refresh_token: None,
        valid_until,
    }
    .insert(&state.db_pool)
    .await?;

    // Display the code to the user if the redirect URI is "urn:ietf:wg:oauth:2.0:oob"
    if redirect_uri == "urn:ietf:wg:oauth:2.0:oob" {
        let page = TokenTemplate {
            token: access_token.access_token,
        }
        .render()?;

        Ok(Html(page).into_response())
    } else {
        let response = AccessTokenResponse {
            access_token: access_token.access_token,
            created_at: ACCESS_TOKEN_VALID_DURATION.whole_seconds(),
            ..AccessTokenResponse::default()
        };

        Ok(Json(&response).into_response())
    }
}

async fn password_grant(
    state: &ArcState,
    FormPasswordGrant {
        username, password, ..
    }: FormPasswordGrant,
) -> Result<impl IntoResponse, Error> {
    let actor = Actor::by_username_local(&state.db_pool, username.as_str()).await?;
    if !password::verify(password, actor.password_hash.unwrap()).await {
        return Err(Error::Unauthorized);
    }

    let valid_until = OffsetDateTime::now_utc() + ACCESS_TOKEN_VALID_DURATION;
    let access_token = crate::crypto::token::generate();

    let access_token = InsertOAuthToken {
        application_id: None,
        actor_id: actor.id,
        access_token,
        refresh_token: None,
        valid_until,
    }
    .insert(&state.db_pool)
    .await?;

    let response = AccessTokenResponse {
        access_token: access_token.access_token,
        created_at: access_token.created_at.unix_timestamp(),
        ..AccessTokenResponse::default()
    };

    Ok(Json(response))
}

#[debug_handler]
pub async fn token(
    Extension(state): Extension<ArcState>,
    Form(form): Form<TokenForm>,
) -> Result<impl IntoResponse, Error> {
    let response = match form.grant_type.as_str() {
        "authorization_code" => {
            let form_data = form.data.code_grant()?;
            code_grant(&state, form_data).await?.into_response()
        }
        "password" => {
            let form_data = form.data.password_grant()?;
            password_grant(&state, form_data).await?.into_response()
        }
        _ => return Err(Error::InvalidRequest),
    };

    Ok(response)
}
