use {
    super::{TokenTemplate, AUTHORIZE_FORM},
    crate::{
        crypto::password,
        database::{Actor, InsertExt, InsertOAuthAuthorization, OAuthApplication},
        error::Error,
        map_err,
    },
    askama::Template,
    chrono::Duration,
    once_cell::sync::Lazy,
    serde::Deserialize,
    std::convert::TryFrom,
    uuid::Uuid,
    warp::{http::Uri, reply::Response, Rejection, Reply},
};

static AUTHORIZATION_CODE_VALIDITY: Lazy<Duration> = Lazy::new(|| Duration::minutes(5));

#[derive(Deserialize)]
pub struct Form {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct Query {
    response_type: String,
    client_id: Uuid,
    redirect_uri: String,
    // scope: Option<String>,
    #[serde(default)]
    state: String,
}

pub async fn get() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::html(AUTHORIZE_FORM.as_str()))
}

pub async fn post(form: Form, query: Query) -> Result<Response, Rejection> {
    let state = crate::state::get();
    let actor = Actor::by_username_local(&state.db_pool, &form.username).await?;
    if !password::verify(form.password, actor.password_hash.unwrap()).await {
        return Err(Error::Unauthorized.into());
    }

    // RFC 6749:
    // ```
    // response_type
    //    REQUIRED.  Value MUST be set to "code".
    // ```
    if query.response_type != "code" {
        return Err(Error::InvalidRequest.into());
    }

    let client = map_err!(OAuthApplication::by_client_id(&state.db_pool, &query.client_id).await)?;
    if client.redirect_uris != query.redirect_uri {
        return Err(Error::InvalidRequest.into());
    }

    let authorization_code = crate::crypto::token::generate()?;

    let validity_duration = *AUTHORIZATION_CODE_VALIDITY;
    let valid_until = chrono::Utc::now() + validity_duration;

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
        let page = map_err!(TokenTemplate {
            token: authorization_code.code,
        }
        .render())?;

        Ok(warp::reply::html(page).into_response())
    } else {
        let redirect_uri = format!(
            "{}?code={}&state={}",
            query.redirect_uri, authorization_code.code, query.state,
        );

        #[allow(clippy::map_err_ignore)]
        let redirect_uri: Uri = Uri::try_from(redirect_uri).map_err(|_| Error::InvalidRequest)?;

        Ok(warp::redirect::temporary(redirect_uri).into_response())
    }
}
