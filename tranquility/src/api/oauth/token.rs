use {
    super::TokenTemplate,
    crate::{
        crypto::password,
        database::{
            model::{InsertOAuthToken, OAuthApplication, OAuthAuthorization},
            InsertExt,
        },
        error::Error,
        map_err,
        state::ArcState,
    },
    askama::Template,
    chrono::Duration,
    once_cell::sync::Lazy,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    warp::{reply::Response, Rejection, Reply},
};

static ACCESS_TOKEN_VALID_DURATION: Lazy<Duration> = Lazy::new(|| Duration::hours(1));

#[derive(Deserialize)]
struct FormPasswordGrant {
    username: String,
    password: String,
}

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
pub struct Form {
    grant_type: String,

    #[serde(flatten)]
    data: FormData,
}

impl FormData {
    pub fn code_grant(self) -> Result<FormCodeGrant, Rejection> {
        match self {
            Self::CodeGrant(form) => Ok(form),
            _ => Err(Error::InvalidRequest.into()),
        }
    }

    pub fn password_grant(self) -> Result<FormPasswordGrant, Rejection> {
        match self {
            Self::PasswordGrant(form) => Ok(form),
            _ => Err(Error::InvalidRequest.into()),
        }
    }
}

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
) -> Result<Response, Rejection> {
    let client = map_err!(OAuthApplication::by_client_id(&state.db_pool, &client_id).await)?;
    if client.client_secret != client_secret || client.redirect_uris != redirect_uri {
        return Err(Error::Unauthorized.into());
    }

    let authorization_code = map_err!(OAuthAuthorization::by_code(&state.db_pool, &code).await)?;

    let valid_until = *ACCESS_TOKEN_VALID_DURATION;
    let valid_until = chrono::Utc::now() + valid_until;

    let access_token = crate::crypto::token::generate()?;

    let access_token = map_err!(
        InsertOAuthToken {
            application_id: Some(client.id),
            actor_id: authorization_code.actor_id,
            access_token,
            refresh_token: None,
            valid_until: valid_until.naive_utc(),
        }
        .insert(&state.db_pool)
        .await
    )?;

    // Display the code to the user if the redirect URI is "urn:ietf:wg:oauth:2.0:oob"
    if redirect_uri == "urn:ietf:wg:oauth:2.0:oob" {
        let page = map_err!(TokenTemplate {
            token: access_token.access_token,
        }
        .render())?;

        Ok(warp::reply::html(page).into_response())
    } else {
        let response = AccessTokenResponse {
            access_token: access_token.access_token,
            created_at: ACCESS_TOKEN_VALID_DURATION.num_seconds(),
            ..AccessTokenResponse::default()
        };

        Ok(warp::reply::json(&response).into_response())
    }
}

async fn password_grant(
    state: &ArcState,
    FormPasswordGrant {
        username, password, ..
    }: FormPasswordGrant,
) -> Result<impl Reply, Rejection> {
    let actor =
        crate::database::actor::select::by_username_local(&state.db_pool, username.as_str())
            .await?;
    if !password::verify(password, actor.password_hash.unwrap()).await {
        return Err(Error::Unauthorized.into());
    }

    let valid_until = *ACCESS_TOKEN_VALID_DURATION;
    let valid_until = chrono::Utc::now() + valid_until;

    let access_token = crate::crypto::token::generate()?;

    let access_token = map_err!(
        InsertOAuthToken {
            application_id: None,
            actor_id: actor.id,
            access_token,
            refresh_token: None,
            valid_until: valid_until.naive_utc(),
        }
        .insert(&state.db_pool)
        .await
    )?;

    let response = AccessTokenResponse {
        access_token: access_token.access_token,
        created_at: access_token.created_at.timestamp(),
        ..AccessTokenResponse::default()
    };

    Ok(warp::reply::json(&response))
}

pub async fn token(state: ArcState, form: Form) -> Result<Response, Rejection> {
    let response = match form.grant_type.as_str() {
        "authorization_code" => {
            let form_data = form.data.code_grant()?;
            code_grant(&state, form_data).await?.into_response()
        }
        "password" => {
            let form_data = form.data.password_grant()?;
            password_grant(&state, form_data).await?.into_response()
        }
        _ => return Err(Error::InvalidRequest.into()),
    };

    Ok(response)
}
