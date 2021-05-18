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
