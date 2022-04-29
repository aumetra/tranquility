use argon2::Error as Argon2Error;
use askama::Error as AskamaError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::{header::InvalidHeaderValue as ReqwestInvalidHeaderValue, Error as ReqwestError};
use rsa::{errors::Error as RsaError, pkcs8::Error as Pkcs8Error};
use serde_json::Error as SerdeJsonError;
use sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError};
use tranquility_http_signatures::Error as HttpSignaturesError;
use url::ParseError as UrlParseError;
use uuid::Error as UuidError;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
/// Combined error enum for converting errors into rejections
pub enum Error {
    #[error("argon2 operation failed")]
    Argon2(#[from] Argon2Error),

    #[error("Template formatting failed: {0}")]
    Askama(#[from] AskamaError),

    #[error("Remote content fetch failed")]
    Fetch,

    #[error("HTTP signature operation failed: {0}")]
    HttpSignatures(#[from] HttpSignaturesError),

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Malformed URL")]
    MalformedUrl,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("PKCS#8 operation failed: {0}")]
    Pkcs8(#[from] Pkcs8Error),

    #[error("reqwest operation failed: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Invalid reqwest HeaderValue: {0}")]
    ReqwestInvalidHeaderValue(#[from] ReqwestInvalidHeaderValue),

    #[error("RSA operation failed: {0}")]
    Rsa(#[from] RsaError),

    #[error("Database operation failed: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("Database migration failed: {0}")]
    SqlxMigration(#[from] SqlxMigrationError),

    #[error("serde-json operation failed: {0}")]
    SerdeJson(#[from] SerdeJsonError),

    #[error("Unexpected webfinger resource")]
    UnexpectedWebfingerResource,

    #[error("Unknown activity")]
    UnknownActivity,

    #[error("URL couldn't be parsed: {0}")]
    UrlParse(#[from] UrlParseError),

    #[error("UUID operation failed: {0}")]
    Uuid(#[from] UuidError),

    #[error("Validation error")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let error_text = self.to_string();

        match self {
            Error::InvalidRequest
            | Error::UnknownActivity
            | Error::MalformedUrl
            | Error::Uuid(..) => (StatusCode::BAD_REQUEST, error_text).into_response(),

            // Add special case to send the previously defined error messages
            Error::Validation(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),

            Error::Unauthorized => (StatusCode::UNAUTHORIZED, error_text).into_response(),

            Error::Argon2(..)
            | Error::Pkcs8(..)
            | Error::Sqlx(..)
            | Error::SqlxMigration(..)
            | Error::Rsa(..) => {
                error!(error = ?self, "Internal error occurred");

                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }

            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
