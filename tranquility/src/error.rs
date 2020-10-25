use {
    argon2::Error as Argon2Error,
    http_signatures::Error as HttpSignaturesError,
    openssl::error::ErrorStack as OpensslErrorStack,
    reqwest::{header::InvalidHeaderValue as ReqwestInvalidHeaderValue, Error as ReqwestError},
    serde_json::Error as SerdeJsonError,
    sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError},
    std::error::Error as StdError,
    thiserror::Error as DeriveError,
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

    #[error("Username taken")]
    DuplicateUsername,

    #[error("Remote content fetch failed")]
    Fetch,

    #[error("An error occurred")]
    General(#[from] Box<dyn StdError + Send + Sync>),

    #[error("HTTP signatures error")]
    HttpSignatures(#[from] HttpSignaturesError),

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

    #[error("Unknown activity")]
    UnknownActivity,

    #[error("Unknown key identifier")]
    UnknownKeyIdentifier,

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
