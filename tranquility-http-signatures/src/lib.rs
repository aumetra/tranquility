#![forbid(rust_2018_idioms, unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use {
    crate::{error::Result, signature::Signature, sigstr::SignatureString},
    http::header::{HeaderName, HeaderValue},
    once_cell::sync::Lazy,
};

static SIGNATURE_HEADER: Lazy<HeaderName> = Lazy::new(|| HeaderName::from_static("signature"));

/// Sign an HTTP request
pub fn sign<'r, 'k, R, K>(req: R, priv_key: K) -> Result<(HeaderName, HeaderValue)>
where
    R: Into<Request<'r>>,
    K: Into<PrivateKey<'k>>,
{
    __into!(req, priv_key);
}

/// Verify an HTTP request
pub fn verify<'r, 'p, R, K>(req: R, pub_key: K) -> Result<bool>
where
    R: Into<Request<'r>>,
    K: Into<PublicKey<'p>>,
{
    __into!(req, pub_key);

    // Parse the signature header
    let signature = req.signature_string()?;
    let signature = Signature::parse(signature)?;

    // Build a signature string
    let signature_string = SignatureString::build(&req, &signature.headers)?;
}

mod error;
mod hash;
mod key;
mod macros;
mod request;
mod signature;
mod sigstr;
mod util;

#[cfg(test)]
mod tests;

pub use {
    error::Error,
    key::{PrivateKey, PublicKey},
    request::Request,
};
