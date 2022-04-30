#![forbid(rust_2018_idioms, unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
// Disable this clippy lint, otherwise clippy will complain when compiled in a test environment
// (for example, with rust-analyzer)
#![cfg_attr(test, allow(clippy::unnecessary_wraps))]

//!
//! Implementation of the HTTP signatures [spec](https://datatracker.ietf.org/doc/html/draft-cavage-http-signatures-12#appendix-C.1)
//!

use crate::{alg::Algorithm, error::Result, signature::Signature, sigstr::SignatureString};
use http::header::{HeaderName, HeaderValue};

static SIGNATURE: HeaderName = HeaderName::from_static("signature");

/// Sign an HTTP request
pub fn sign<'r, 'k, R, K>(
    req: R,
    headers: &[&str],
    priv_key: K,
) -> Result<(HeaderName, HeaderValue)>
where
    R: Into<Request<'r>>,
    K: Into<PrivateKey<'k>>,
{
    __into!(req, priv_key);

    // Build a signature string
    let signature_string = SignatureString::build(&req, headers)?;
    let encoded_signature_string = signature_string.to_string();
    let signature_string_bytes = encoded_signature_string.as_bytes();

    // Use the default algorithm for signing
    let algorithm = Algorithm::default();

    // Decode the private key
    let decoded_priv_key = pem::decode(priv_key.data)?;

    // Sign the signature string and base64-encode the signature
    let signature = algorithm.sign(decoded_priv_key, signature_string_bytes)?;
    let encoded_signature = base64::encode(signature);

    // Build the signature header and encode it into an `HeaderValue`
    let signature_header = Signature::new(
        priv_key.key_id,
        None,
        headers.to_vec(),
        &encoded_signature,
        None,
        None,
    );
    let signature_header = signature_header.encode()?;

    Ok((SIGNATURE.clone(), signature_header))
}

/// Verify an HTTP request
pub fn verify<'r, 'p, R, K>(req: R, pub_key: K) -> Result<bool>
where
    R: Into<Request<'r>>,
    K: Into<PublicKey<'p>>,
{
    __into!(req, pub_key);

    // Parse the signature header
    let signature = req.signature()?;
    let signature = Signature::parse(signature)?;

    // Build a signature string
    let signature_string = SignatureString::build(&req, &signature.headers)?;
    let encoded_signature_string = signature_string.to_string();
    let signature_string_bytes = encoded_signature_string.as_bytes();

    // Parse the algorithm and public key
    let algorithm = Algorithm::parse(signature.algorithm)?;
    let decoded_pub_key = pem::decode(pub_key)?;
    let public_key = algorithm.prepare_public_key(decoded_pub_key)?;

    // Decode the base64-encoded signature
    let decoded_signature = base64::decode(signature.signature)?;

    // Prepare the public key and verify the signature
    let is_valid = public_key
        .verify(signature_string_bytes, &decoded_signature)
        .is_ok();

    Ok(is_valid)
}

mod alg;
mod error;
mod key;
mod macros;
mod pem;
mod request;
mod signature;
mod sigstr;
mod util;

#[cfg(test)]
mod tests;

pub use self::error::Error;
pub use self::key::{PrivateKey, PublicKey};
pub use self::request::Request;
