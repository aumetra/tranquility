use crate::{
    error::{Error, Result},
    SIGNATURE,
};
use http::header::{HeaderMap, AUTHORIZATION};

/// HTTP request that is being processed
pub struct Request<'a> {
    /// Method of the HTTP request
    pub(crate) method: &'a str,

    /// Requested path
    pub(crate) path: &'a str,

    /// Optional query of the request
    pub(crate) query: Option<&'a str>,

    /// The headers of the HTTP request
    pub(crate) headers: &'a HeaderMap,
}

impl<'a> Request<'a> {
    #[must_use]
    /// Construct a new request
    pub fn new(
        method: &'a str,
        path: &'a str,
        query: Option<&'a str>,
        headers: &'a HeaderMap,
    ) -> Self {
        Self {
            method,
            path,
            query,
            headers,
        }
    }

    /// Get the signature from the HTTP request
    pub(crate) fn signature(&self) -> Result<&str> {
        // Try to get the signature from the signature header
        if let Some(header_value) = self.headers.get(&SIGNATURE) {
            return Ok(header_value.to_str()?);
        }

        // Try to get the signature from the authorization header
        if let Some(header_value) = self.headers.get(AUTHORIZATION) {
            let header_value_str = header_value.to_str()?;

            // Split off the `Signature`
            let first_space_pos = header_value_str.find(' ').ok_or(Error::InvalidHeader)?;
            let (_, header_value_str) = header_value_str.split_at(first_space_pos);

            return Ok(header_value_str);
        }

        Err(Error::MissingSignatureHeader)
    }
}

impl<'a, T> From<&'a http::Request<T>> for Request<'a> {
    fn from(req: &'a http::Request<T>) -> Self {
        let method = req.method().as_str();
        let headers = req.headers();

        let uri = req.uri();
        let path = uri.path();
        let query = uri.query();

        Self::new(method, path, query, headers)
    }
}

#[cfg(feature = "reqwest")]
impl<'a> From<&'a reqwest::Request> for Request<'a> {
    fn from(req: &'a reqwest::Request) -> Self {
        let method = req.method().as_str();
        let headers = req.headers();

        let uri = req.url();
        let path = uri.path();
        let query = uri.query();

        Self::new(method, path, query, headers)
    }
}
