use {
    crate::{
        error::{Error, Result},
        wrap_cow, wrap_cow_option, SIGNATURE_HEADER,
    },
    http::header::{HeaderMap, AUTHORIZATION},
    std::borrow::Cow,
};

/// HTTP request that is being processed
pub struct Request<'a> {
    /// Method of the HTTP request
    pub(crate) method: Cow<'a, str>,

    /// Requested path
    pub(crate) path: Cow<'a, str>,

    /// Optional query of the request
    pub(crate) query: Option<Cow<'a, str>>,

    /// The headers of the HTTP request
    pub(crate) headers: Cow<'a, HeaderMap>,
}

impl<'a> Request<'a> {
    #[must_use]
    /// Construct a new request
    pub fn new(
        method: Cow<'a, str>,
        path: Cow<'a, str>,
        query: Option<Cow<'a, str>>,
        headers: Cow<'a, HeaderMap>,
    ) -> Self {
        Self {
            method,
            path,
            query,
            headers,
        }
    }

    /// Get the signature string from the HTTP request
    pub(crate) fn signature_string(&self) -> Result<&str> {
        let headers = self.headers.as_ref();

        // Try to get the signature string from the signature header
        if let Some(header_value) = headers.get(&*SIGNATURE_HEADER) {
            return Ok(header_value.to_str()?.trim());
        }

        // Try to get the signature string from the authorization header
        if let Some(header_value) = headers.get(AUTHORIZATION) {
            let header_value_str = header_value.to_str()?;
            let header_value_str = header_value_str.trim_start_matches("signature").trim();

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

        // Wrap the variables in `Cow`s
        wrap_cow!(Borrowed; method, headers, path);
        wrap_cow_option!(Borrowed; query);

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

        // Wrap the variables in `Cow`s
        wrap_cow!(Borrowed; method, headers, path);
        wrap_cow_option!(Borrowed; query);

        Self::new(method, path, query, headers)
    }
}
