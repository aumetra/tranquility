use {
    serde_json::Error as SerdeJsonError,
    std::fmt,
    tranquility_http_signatures::Error as HttpSignaturesError,
    url::ParseError as UrlParseError,
    uuid::Error as UuidError,
    warp::{
        http::StatusCode,
        reject::{Reject, Rejection},
        reply::Response,
        Reply,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Remote content fetch failed")]
    Fetch,

    #[error("HTTP signature operation failed")]
    HttpSignatures(#[from] HttpSignaturesError),

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Malformed URL")]
    MalformedUrl,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("serde-json operation failed")]
    SerdeJson(#[from] SerdeJsonError),

    #[error("Unexpected webfinger resource")]
    UnexpectedWebfingerResource,

    #[error("Unknown activity")]
    UnknownActivity,

    #[error("URL couldn't be parsed")]
    UrlParse(#[from] UrlParseError),

    #[error("UUID operation failed")]
    Uuid(#[from] UuidError),
}

impl Reject for Error {}

pub trait IntoRejection<T> {
    /// Convert a `Result<T, E>` into an `Result<T, Rejection>`
    fn into_rejection(self) -> Result<T, Rejection>;
}

impl<T, E> IntoRejection<T> for Result<T, E>
where
    E: fmt::Display + Sized,
{
    fn into_rejection(self) -> Result<T, Rejection> {
        self.map_err(|e| {
            let msg = format!("{}", e);

            Error::Custom(msg).into()
        })
    }
}

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

            Error::Unauthorized => {
                Ok(warp::reply::with_status(error_text, StatusCode::UNAUTHORIZED).into_response())
            }

            _ => Err(rejection),
        }
    } else {
        Err(rejection)
    }
}
