pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// Unified error enum
pub enum Error {
    #[error("Base64 decode error: {0}")]
    /// Base64 decode error
    Base64Decode(#[from] base64::DecodeError),

    #[error("HTTP ToStrError: {0}")]
    /// HTTP ToStrError
    HttpToStr(#[from] http::header::ToStrError),

    #[error("Invalid algorithm")]
    /// Invalid algorithm
    InvalidAlgorithm,

    #[error("Invalid header value: {0}")]
    /// Invalid HTTP header value
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error("Invalid header")]
    /// Invalid HTTP header
    InvalidHeader,

    #[error("Missing header")]
    /// Missing HTTP header
    MissingHeader,

    #[error("Missing signature header")]
    /// Missing signature header
    MissingSignatureHeader,

    #[error("Missing signature field")]
    /// Missing signature field
    MissingSignatureField,

    #[error("PEM error: {0}")]
    /// PEM decoding error
    Pem(#[from] pem::PemError),

    #[error("Ring error")]
    /// Ring crypto error
    Ring(#[from] ring::error::Unspecified),

    #[error("Invalid key: {0}")]
    /// Ring invalid key
    RingInvalidKey(#[from] ring::error::KeyRejected),

    #[error("Unknown algorithm")]
    /// Unknown algorithm
    UnknownAlgorithm,

    #[error("Unknown key type")]
    /// Unknown key type
    UnknownKeyType,
}
