#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use {
    base64::DecodeError as Base64DecodeError,
    http::header::{
        HeaderMap, HeaderName, HeaderValue, InvalidHeaderName as HttpInvalidHeaderName,
        InvalidHeaderValue as HttpInvalidHeaderValue, ToStrError as HttpToStrError,
    },
    pem::PemError,
    rsa::{errors::Error as RsaError, Hash, PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey},
    sha2::{Digest, Sha256, Sha384, Sha512},
    std::collections::HashMap,
};

const STANDARD_HASH: Hash = Hash::SHA2_256;

trait HashExt {
    fn to_algorithm(self) -> String;

    fn from_signature_string(signature_string: &HashMap<&str, &str>) -> Self;

    fn calculate(self, data: &[u8]) -> Vec<u8>;
}

impl HashExt for Hash {
    fn to_algorithm(self) -> String {
        let hash = match self {
            Hash::SHA2_256 => "sha256",
            Hash::SHA2_384 => "sha384",
            Hash::SHA2_512 => "sha512",
            _ => unreachable!(),
        };

        format!("rsa-{}", hash)
    }

    fn from_signature_string(signature_string: &HashMap<&str, &str>) -> Self {
        match signature_string.get("algorithm") {
            Some(val) => {
                let message_digest_str = val.split('-').last().unwrap();

                match message_digest_str {
                    "sha384" => Hash::SHA2_384,
                    "sha512" => Hash::SHA2_512,
                    _ => Hash::SHA2_256,
                }
            }
            None => Hash::SHA2_256,
        }
    }

    fn calculate(self, data: &[u8]) -> Vec<u8> {
        match self {
            Hash::SHA2_256 => Sha256::digest(data).to_vec(),
            Hash::SHA2_384 => Sha384::digest(data).to_vec(),
            Hash::SHA2_512 => Sha512::digest(data).to_vec(),
            _ => unreachable!(),
        }
    }
}

/// Combined error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("base64 decoding error")]
    Base64Decode(#[from] Base64DecodeError),

    #[error("HTTP value to string conversion failed")]
    HttpToStr(#[from] HttpToStrError),

    #[error("Invalid header name")]
    HttpInvalidHeaderName(#[from] HttpInvalidHeaderName),

    #[error("Invalid header value")]
    HttpInvalidHeaderValue(#[from] HttpInvalidHeaderValue),

    #[error("Malformed signature header")]
    MalformedSignatureHeader,

    #[error("Missing signature header")]
    MissingSignatureHeader,

    #[error("PEM operation failed")]
    Pem(#[from] PemError),

    #[error("RSA operation failed")]
    Rsa(#[from] RsaError),

    #[error("This key type is not supported")]
    UnsupportedKeyType,
}

pub struct HttpRequest<'a> {
    method: &'a str,
    path: &'a str,
    query: Option<&'a str>,
    headers: &'a HeaderMap,
}

impl<'a> HttpRequest<'a> {
    pub fn new(
        method: &'a str,
        path: &'a str,
        query: Option<&'a str>,
        headers: &'a HeaderMap,
    ) -> HttpRequest<'a> {
        HttpRequest {
            method,
            path,
            query,
            headers,
        }
    }
}

#[cfg(feature = "reqwest")]
impl<'a> From<&'a reqwest::Request> for HttpRequest<'a> {
    fn from(req: &'a reqwest::Request) -> Self {
        HttpRequest {
            method: req.method().as_str(),
            path: req.url().path(),
            query: req.url().query(),
            headers: req.headers(),
        }
    }
}

enum SignaturePart<'a> {
    RequestTarget(&'a str, &'a str, Option<&'a str>),
    Header(&'a str, &'a HeaderValue),
}

impl<'a> SignaturePart<'a> {
    fn format(self) -> Result<String, Error> {
        let signature_part = match self {
            SignaturePart::RequestTarget(method, path, query) => {
                let query = query.map(|query| format!("?{}", query)).unwrap_or_default();

                format!(
                    "(request-target): {} {}{}",
                    method.to_lowercase(),
                    path,
                    query
                )
            }
            SignaturePart::Header(name, value) => format!("{}: {}", name, value.to_str()?),
        };

        Ok(signature_part)
    }
}

fn build_signature_header(
    key_id: &str,
    header_names: &[&str],
    hash: Hash,
    private_key: &RSAPrivateKey,
    signature_string: &str,
) -> Result<HeaderValue, Error> {
    let hashed_signature_string = hash.calculate(signature_string.as_bytes());

    let padding_scheme = PaddingScheme::PKCS1v15Sign { hash: Some(hash) };
    let signature = private_key.sign(padding_scheme, hashed_signature_string.as_slice())?;
    let encoded_signature = base64::encode(signature);

    let algorithm = hash.to_algorithm();
    let signature_header = HeaderValue::from_str(&format!(
        "keyId=\"{}\",algorithm=\"{}\",headers=\"{}\",signature=\"{}\"",
        key_id,
        algorithm.as_str(),
        header_names.join(" "),
        encoded_signature
    ))?;

    Ok(signature_header)
}

fn build_signature_string(signature_parts: Vec<SignaturePart>) -> Result<String, Error> {
    let signature_string = signature_parts
        .into_iter()
        .map(SignaturePart::format)
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    Ok(signature_string)
}

fn parse_signature_header<'a>(
    request: &HttpRequest<'a>,
    signature_header: &'a HeaderValue,
) -> Result<(Vec<SignaturePart<'a>>, Hash, Vec<u8>), Error> {
    let signature_header = signature_header.to_str()?.trim_start_matches("Signature ");

    let parsed_signature_string = signature_header
        .split(',')
        .filter_map(|kv_pair| {
            let (key, value) = kv_pair.split_at(kv_pair.find('=')?);
            // Skip the first character because the first character is the '='
            let value = &value[1..];

            let key = key.trim();
            let value = value.trim_matches('"');

            Some((key, value))
        })
        .collect::<HashMap<_, _>>();

    let decoded_signature = base64::decode(parsed_signature_string["signature"])?;
    let mut signature_parts = parsed_signature_string
        .get("headers")
        // `Default` isn't implemented for `&&str`
        .unwrap_or(&"")
        .split_whitespace()
        .filter_map(|header| match header {
            "(request-target)" => Some(SignaturePart::RequestTarget(
                request.method,
                request.path,
                request.query,
            )),
            "(created)" => None,
            "(expires)" => {
                // Theoretically we would have to check if the signature has expired
                None
            }
            header => {
                let value = request.headers.get(header)?;

                Some(SignaturePart::Header(header, value))
            }
        })
        .collect::<Vec<_>>();

    // If a list of headers isn't included, only the "date" header is used
    // See draft-cavage-http-signatures-11#Appendix.C.1
    if signature_parts.is_empty() {
        let date_header = request
            .headers
            .get("date")
            .ok_or(Error::MalformedSignatureHeader)?;

        signature_parts.push(SignaturePart::Header("date", date_header));
    }

    let hash = Hash::from_signature_string(&parsed_signature_string);

    Ok((signature_parts, hash, decoded_signature))
}

/// Generate a signature for the HTTP request
///
/// # Errors
///
/// See the `Error` struct
pub fn sign<'a, T: Into<HttpRequest<'a>>>(
    request: T,
    key_id: &str,
    header_names: &[&str],
    private_key: &[u8],
) -> Result<(HeaderName, HeaderValue), Error> {
    let request = request.into();

    let header_name = HeaderName::from_static("signature");
    let private_key = parse_private_key(private_key)?;

    let signature_parts = header_names
        .iter()
        .map(|header_name| {
            if *header_name == "(request-target)" {
                SignaturePart::RequestTarget(request.method, request.path, request.query)
            } else {
                let value = &request.headers[*header_name];

                SignaturePart::Header(*header_name, value)
            }
        })
        .collect::<Vec<_>>();

    let signature_string = build_signature_string(signature_parts)?;
    let signature_header = build_signature_header(
        key_id,
        header_names,
        STANDARD_HASH,
        &private_key,
        &signature_string,
    )?;

    Ok((header_name, signature_header))
}

fn parse_private_key(data: &[u8]) -> Result<RSAPrivateKey, Error> {
    let parsed_pem = pem::parse(data)?;
    let contents = parsed_pem.contents.as_slice();

    match parsed_pem.tag.as_str() {
        "RSA PRIVATE KEY" => RSAPrivateKey::from_pkcs1(contents).map_err(Error::from),
        "PRIVATE KEY" => RSAPrivateKey::from_pkcs8(contents).map_err(Error::from),
        _ => Err(Error::UnsupportedKeyType),
    }
}

fn parse_public_key(data: &[u8]) -> Result<RSAPublicKey, Error> {
    let parsed_pem = pem::parse(data)?;
    let contents = parsed_pem.contents.as_slice();

    match parsed_pem.tag.as_str() {
        "RSA PUBLIC KEY" => RSAPublicKey::from_pkcs1(contents).map_err(Error::from),
        "PUBLIC KEY" => RSAPublicKey::from_pkcs8(contents).map_err(Error::from),
        _ => Err(Error::UnsupportedKeyType),
    }
}

/// Verify the HTTP signature
///
/// # Errors
///
/// See the `Error` struct
pub fn verify<'a, T: Into<HttpRequest<'a>>>(request: T, public_key: &[u8]) -> Result<bool, Error> {
    let request = request.into();

    let signature_header = match request.headers.get("authorization") {
        Some(val) if val.to_str()?.starts_with("Signature") => val,
        _ => request
            .headers
            .get("signature")
            .ok_or(Error::MissingSignatureHeader)?,
    };
    let public_key = parse_public_key(public_key)?;

    let (signature_parts, hash, decoded_signature) =
        parse_signature_header(&request, &signature_header)?;
    let signature_string = build_signature_string(signature_parts)?;

    let hashed_signature_string = hash.calculate(signature_string.as_bytes());
    let signature_valid = public_key.verify(
        PaddingScheme::PKCS1v15Sign { hash: Some(hash) },
        hashed_signature_string.as_slice(),
        decoded_signature.as_slice(),
    );

    match signature_valid {
        Ok(()) => Ok(true),
        Err(RsaError::Verification) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests;
