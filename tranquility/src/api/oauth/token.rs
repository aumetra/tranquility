use {
    super::TokenTemplate,
    crate::{crypto::password, error::Error},
    askama::Template,
    chrono::Duration,
    once_cell::sync::Lazy,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    warp::{reply::Response, Rejection, Reply},
};

static ACCESS_TOKEN_VALID_DURATION: Lazy<Duration> = Lazy::new(|| Duration::hours(1));

#[derive(Deserialize)]
pub struct Form {
    grant_type: String,

    client_id: Option<Uuid>,
    client_secret: Option<String>,
    redirect_uri: Option<String>,
    // scope: Option<String>,
    code: Option<String>,

    username: Option<String>,
    password: Option<String>,
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
    client_id: Uuid,
    client_secret: String,
    redirect_uri: String,
    code: String,
) -> Result<Response, Rejection> {
    let client = crate::database::oauth::application::select::by_client_id(&client_id).await?;
    if client.client_secret != client_secret || client.redirect_uris != redirect_uri {
        return Err(Error::Unauthorized.into());
    }

    let authorization_code = crate::database::oauth::authorization::select::by_code(&code).await?;

    let valid_until = *ACCESS_TOKEN_VALID_DURATION;
    let valid_until = chrono::Utc::now() + valid_until;

    let access_token = crate::crypto::token::generate()?;
    let access_token = crate::database::oauth::token::insert(
        Some(client.id),
        authorization_code.actor_id,
        access_token,
        None,
        valid_until.naive_utc(),
    )
    .await?;

    // Display the code to the user if the redirect URI is "urn:ietf:wg:oauth:2.0:oob"
    if redirect_uri == "urn:ietf:wg:oauth:2.0:oob" {
        let page = TokenTemplate {
            token: access_token.access_token,
        }
        .render()
        .map_err(Error::from)?;

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

async fn password_grant(username: String, password: String) -> Result<impl Reply, Rejection> {
    let actor = crate::database::actor::select::by_username_local(username.as_str()).await?;
    if !password::verify(password, actor.password_hash.unwrap()).await {
        return Err(Error::Unauthorized.into());
    }

    let valid_until = *ACCESS_TOKEN_VALID_DURATION;
    let valid_until = chrono::Utc::now() + valid_until;

    let access_token = crate::crypto::token::generate()?;
    let access_token = crate::database::oauth::token::insert(
        None,
        actor.id,
        access_token,
        None,
        valid_until.naive_utc(),
    )
    .await?;

    let response = AccessTokenResponse {
        access_token: access_token.access_token,
        created_at: access_token.created_at.timestamp(),
        ..AccessTokenResponse::default()
    };

    Ok(warp::reply::json(&response))
}

pub async fn token(form: Form) -> Result<Response, Rejection> {
    let response = match form.grant_type.as_str() {
        "authorization_code" => code_grant(
            form.client_id.unwrap(),
            form.client_secret.unwrap(),
            form.redirect_uri.unwrap(),
            form.code.unwrap(),
        )
        .await?
        .into_response(),
        "password" => password_grant(form.username.unwrap(), form.password.unwrap())
            .await?
            .into_response(),
        _ => return Err(Error::InvalidRequest.into()),
    };

    Ok(response)
}
