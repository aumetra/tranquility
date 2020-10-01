use argon2::Error as Argon2Error;
use openssl::error::ErrorStack as OpensslErrorStack;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError};
use std::error::Error as StdError;
use thiserror::Error as DeriveError;
use uuid::Error as UuidError;
use validator::ValidationErrors;
use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    Reply,
};

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2Error(#[from] Argon2Error),

    #[error("Username taken")]
    DuplicateUsername,

    #[error("Remote content fetch failed")]
    FetchError,

    #[error("An error occurred")]
    GeneralError(#[from] Box<dyn StdError + Send + Sync>),

    #[error("Invalid HTTP signature")]
    InvalidHTTPSignature,

    #[error("OpenSSL operation failed")]
    OpensslError(#[from] OpensslErrorStack),

    #[error("reqwest operation failed")]
    ReqwestError(#[from] ReqwestError),

    #[error("Database operation failed")]
    SqlxError(#[from] SqlxError),

    #[error("Database migration failed")]
    SqlxMigrationError(#[from] SqlxMigrationError),

    #[error("serde-json operation failed")]
    SerdeJsonError(#[from] SerdeJsonError),

    #[error("Unknown key identifier")]
    UnknownKeyIdentifier,

    #[error("UUID operation failed")]
    UuidError(#[from] UuidError),

    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
}

impl Reject for Error {}

impl From<Error> for Rejection {
    fn from(err: Error) -> Rejection {
        warp::reject::custom(err)
    }
}

pub async fn recover(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = rejection.find::<Error>() {
        match error {
            Error::InvalidHTTPSignature => Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::UNAUTHORIZED,
            )),
            _ => Err(rejection),
        }
    } else {
        Err(rejection)
    }
}
