pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("HTTP ToStrError: {0}")]
    HttpToStr(#[from] http::header::ToStrError),

    #[error("Invalid algorithm")]
    InvalidAlgorithm,

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error("Invalid header")]
    InvalidHeader,

    #[error("Missing header")]
    MissingHeader,

    #[error("Missing signature header")]
    MissingSignatureHeader,

    #[error("Missing signature field")]
    MissingSignatureField,

    #[error("PEM error: {0}")]
    Pem(#[from] pem::PemError),

    #[error("Ring error")]
    Ring(#[from] ring::error::Unspecified),

    #[error("Invalid key: {0}")]
    RingInvalidKey(#[from] ring::error::KeyRejected),

    #[error("Unknown algorithm")]
    UnknownAlgorithm,

    #[error("Unknown key type")]
    UnknownKeyType,
}
