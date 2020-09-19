use argon2::Error as Argon2Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error as DeriveError;
use warp::reject::{Reject, Rejection};

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2Error(#[from] Argon2Error),

    #[error("reqwest returned an error")]
    ReqwestError(#[from] ReqwestError),

    #[error("serde-json returned an error")]
    SerdeJsonError(#[from] SerdeJsonError),
}

impl Reject for Error {}

impl From<Error> for Rejection {
    fn from(err: Error) -> Rejection {
        warp::reject::custom(err)
    }
}
