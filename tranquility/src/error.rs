use argon2::Error as Argon2Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError};
use thiserror::Error as DeriveError;
use warp::reject::{Reject, Rejection};

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2Error(#[from] Argon2Error),

    #[error("reqwest operation failed")]
    ReqwestError(#[from] ReqwestError),

    #[error("Database operation failed")]
    SqlxError(#[from] SqlxError),

    #[error("Database migration failed")]
    SqlxMigrationError(#[from] SqlxMigrationError),

    #[error("serde-json operation failed")]
    SerdeJsonError(#[from] SerdeJsonError),
}

impl Reject for Error {}

impl From<Error> for Rejection {
    fn from(err: Error) -> Rejection {
        warp::reject::custom(err)
    }
}
