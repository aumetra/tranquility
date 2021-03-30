pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP ToStrError: {0}")]
    HttpToStr(#[from] http::header::ToStrError),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error("Missing header")]
    MissingHeader,

    #[error("Missing signature header")]
    MissingSignatureHeader,

    #[error("Missing signature field")]
    MissingSignatureField,

    #[error("Unknown hash algorithm")]
    UnknownHashAlgorithm,
}
