use {
    crate::map_err,
    argon2::Error as Argon2Error,
    askama::Error as AskamaError,
    reqwest::{header::InvalidHeaderValue as ReqwestInvalidHeaderValue, Error as ReqwestError},
    rsa::errors::Error as RsaError,
    serde_json::Error as SerdeJsonError,
    sqlx::{migrate::MigrateError as SqlxMigrationError, Error as SqlxError},
    tranquility_http_signatures::Error as HttpSignaturesError,
    url::ParseError as UrlParseError,
    uuid::Error as UuidError,
    validator::ValidationErrors,
    warp::{
        http::StatusCode,
        reject::{Reject, Rejection},
        reply::Response,
        Reply,
    },
};

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

impl Reject for Error {}

/// Recover function for recovering from some of the errors with a custom error status
pub async fn recover(rejection: Rejection) -> Result<Response, Rejection> {
    if let Some(error) = rejection.find::<Error>() {
        let error_text = error.to_string();

        match error {
            Error::InvalidRequest
            | Error::UnknownActivity
            | Error::MalformedUrl
            | Error::Uuid(..) => {
                Ok(warp::reply::with_status(error_text, StatusCode::BAD_REQUEST).into_response())
            }

            // Add special case to send the previously defined error messages
            Error::Validation(err) => {
                let response_payload = map_err!(serde_json::to_string(err))?;
                let response = warp::reply::with_status(response_payload, StatusCode::BAD_REQUEST)
                    .into_response();

                Ok(response)
            }

            Error::Unauthorized => {
                Ok(warp::reply::with_status(error_text, StatusCode::UNAUTHORIZED).into_response())
            }

            Error::Argon2(..) | Error::Sqlx(..) | Error::SqlxMigration(..) | Error::Rsa(..) => {
                error!(?error, "Internal error occurred");

                Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }

            _ => Err(rejection),
        }
    } else {
        Err(rejection)
    }
}
