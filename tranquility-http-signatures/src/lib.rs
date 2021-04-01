#![forbid(rust_2018_idioms, unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use {
    crate::{alg::Algorithm, error::Result, signature::Signature, sigstr::SignatureString},
    http::header::{HeaderName, HeaderValue},
    once_cell::sync::Lazy,
};

static SIGNATURE: Lazy<HeaderName> = Lazy::new(|| HeaderName::from_static("signature"));

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

    // Create an algorithm field
    // Giving a "None" defaults to `rsa-sha256`
    let algorithm = Algorithm::parse(None)?;

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

pub mod pem {
    use {
        crate::error::{Error, Result},
        pkcs8::{PrivateKeyDocument, PublicKeyDocument},
    };

    /// Decode PKCS#8 DER to a PKCS#1 DER
    fn decode_pkcs8(data: &[u8], public: bool) -> Result<Vec<u8>> {
        // PKCS#8 is nothing else than PKCS#1 with some additional metadata about the key
        let der_key = if public {
            let pub_key = PublicKeyDocument::from_der(data).map_err(|_| Error::UnknownKeyType)?;

            pub_key.spki().subject_public_key.to_vec()
        } else {
            let priv_key = PrivateKeyDocument::from_der(data).map_err(|_| Error::UnknownKeyType)?;

            priv_key.private_key_info().private_key.to_vec()
        };

        Ok(der_key)
    }

    /// Decode a PEM-encoded key (PKCS#1 or PKCS#8) to PKCS#1 DER
    pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
        let pem = pem::parse(data)?;

        match pem.tag.as_str() {
            "PRIVATE KEY" => decode_pkcs8(pem.contents.as_slice(), false),
            "PUBLIC KEY" => decode_pkcs8(pem.contents.as_slice(), true),
            "RSA PRIVATE KEY" | "RSA PUBLIC KEY" => Ok(pem.contents),
            _ => Err(Error::UnknownKeyType),
        }
    }
}

mod alg;
mod error;
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
