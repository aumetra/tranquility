use {
    argon2::Error as Argon2Error,
    askama::Error as AskamaError,
    http_signatures::Error as HttpSignaturesError,
    openssl::error::ErrorStack as OpensslErrorStack,
    reqwest::{header::InvalidHeaderValue as ReqwestInvalidHeaderValue, Error as ReqwestError},
    serde_json::Error as SerdeJsonError,
    sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError},
    thiserror::Error as DeriveError,
    url::ParseError as UrlParseError,
    uuid::Error as UuidError,
    validator::ValidationErrors,
    warp::{
        http::StatusCode,
        reject::{Reject, Rejection},
        Reply,
    },
};

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2(#[from] Argon2Error),

    #[error("Template formatting failed")]
    Askama(#[from] AskamaError),

    #[error("Username taken")]
    DuplicateUsername,

    #[error("Remote content fetch failed")]
    Fetch,

    #[error("HTTP signatures error")]
    HttpSignatures(#[from] HttpSignaturesError),

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Malformed URL")]
    MalformedUrl,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("OpenSSL operation failed")]
    Openssl(#[from] OpensslErrorStack),

    #[error("reqwest operation failed")]
    Reqwest(#[from] ReqwestError),

    #[error("Invalid reqwest HeaderValue")]
    ReqwestInvalidHeaderValue(#[from] ReqwestInvalidHeaderValue),

    #[error("Database operation failed")]
    Sqlx(#[from] SqlxError),

    #[error("Database migration failed")]
    SqlxMigration(#[from] SqlxMigrationError),

    #[error("serde-json operation failed")]
    SerdeJson(#[from] SerdeJsonError),

    #[error("Unexpected webfinger resource")]
    UnexpectedWebfingerResource,

    #[error("Unknown activity")]
    UnknownActivity,

    #[error("Unknown key identifier")]
    UnknownKeyIdentifier,

    #[error("URL couldn't be parsed")]
    UrlParse(#[from] UrlParseError),

    #[error("UUID operation failed")]
    Uuid(#[from] UuidError),

    #[error("Validation error")]
    Validation(#[from] ValidationErrors),
}

impl Reject for Error {}

impl From<Error> for Rejection {
    fn from(err: Error) -> Self {
        warp::reject::custom(err)
    }
}

fn map_error(error: &Error) -> Result<impl Reply, ()> {
    match error {
        Error::InvalidRequest => Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::BAD_REQUEST,
        )),
        Error::Unauthorized => Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNAUTHORIZED,
        )),
        _ => Err(()),
    }
}

pub async fn recover(rejection: Rejection) -> Result<impl Reply, Rejection> {
    rejection
        .find::<Error>()
        .map_or(Err(()), map_error)
        .map_err(|_| rejection)
}
