use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use headers::{ContentLength, HeaderMapExt};
use std::ops::{Deref, DerefMut};

const KILOBYTES_BYTES: u64 = 1024;
const MEGABYTES_BYTES: u64 = KILOBYTES_BYTES.pow(2);

/// Configuration for the content length limiter
///
/// Put this into the request extensions via the `Extension` layer
#[derive(Clone, Copy, Debug)]
pub struct ContentLengthLimitConfig(u64);

impl ContentLengthLimitConfig {
    /// Set a maximum content length limit in bytes
    pub fn bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Set a maximum content length limit in kilobytes
    pub fn kilobytes(kb: u64) -> Self {
        Self::bytes(kb * KILOBYTES_BYTES)
    }

    /// Set a maximum content length limit in megabytes
    pub fn megabytes(mb: u64) -> Self {
        Self::bytes(mb * MEGABYTES_BYTES)
    }
}

impl Deref for ContentLengthLimitConfig {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Content length limiting extractor wrapping another extractor
///
/// This is a pretty primitive implementation.  
/// It just reads the `Content-Length` header and compares it with the configured maximum value from the configuration stored in the request extensions.
pub struct ContentLengthLimit<T>(pub T);

impl<T> Deref for ContentLengthLimit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ContentLengthLimit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<B, T> FromRequest<B> for ContentLengthLimit<T>
where
    T: FromRequest<B>,
    B: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let content_length_limit = req
            .extensions()
            .get::<ContentLengthLimitConfig>()
            .expect("Content length limit configuration missing from extensions");

        if let Some(ContentLength(content_length)) = req.headers().typed_get::<ContentLength>() {
            if content_length > **content_length_limit {
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!(
                        "Payload exceeded maximum size of {}",
                        **content_length_limit
                    ),
                )
                    .into_response());
            }

            <T as FromRequest<B>>::from_request(req)
                .await
                .map(ContentLengthLimit)
                .map_err(IntoResponse::into_response)
        } else {
            return Err((StatusCode::BAD_REQUEST, "Missing Content-Length header").into_response());
        }
    }
}
