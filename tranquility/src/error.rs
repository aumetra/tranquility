use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error as DeriveError;
use warp::reject::{Reject, Rejection};

#[derive(Debug, DeriveError)]
pub enum Error {
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
