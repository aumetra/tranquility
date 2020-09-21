use argon2::Error as Argon2Error;
use reqwest::Error as ReqwestError;
use rsa::errors::Error as RsaError;
use serde_json::Error as SerdeJsonError;
use sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError};
use std::error::Error as StdError;
use thiserror::Error as DeriveError;
use uuid::Error as UuidError;
use warp::reject::{Reject, Rejection};

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2Error(#[from] Argon2Error),

    #[error("Username taken")]
    DuplicateUsername,

    #[error("An error occurred")]
    GeneralError(#[from] Box<dyn StdError + Send + Sync>),

    #[error("Invalid username")]
    InvalidUsername,

    #[error("reqwest operation failed")]
    ReqwestError(#[from] ReqwestError),

    #[error("RSA operation failed")]
    RSAError(#[from] RsaError),

    #[error("Database operation failed")]
    SqlxError(#[from] SqlxError),

    #[error("Database migration failed")]
    SqlxMigrationError(#[from] SqlxMigrationError),

    #[error("serde-json operation failed")]
    SerdeJsonError(#[from] SerdeJsonError),

    #[error("UUID operation failed")]
    UuidError(#[from] UuidError),
}

impl Reject for Error {}

impl From<Error> for Rejection {
    fn from(err: Error) -> Rejection {
        warp::reject::custom(err)
    }
}
