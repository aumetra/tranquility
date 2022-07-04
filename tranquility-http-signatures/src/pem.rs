use crate::error::{Error, Result};
use pkcs8::{der::Decode, Document, PrivateKeyInfo, SubjectPublicKeyInfo};

/// Convert/Decode PKCS#8 DER to PKCS#1 DER
fn pkcs8_to_pkcs1(data: &[u8], is_public: bool) -> Result<Vec<u8>> {
    // PKCS#8 is nothing else than PKCS#1 with some additional metadata about the key
    let der_key = if is_public {
        let pub_key = Document::from_der(data).map_err(|_| Error::UnknownKeyType)?;
        let pub_key: SubjectPublicKeyInfo<'_> =
            pub_key.decode_msg().map_err(|_| Error::UnknownKeyType)?;

        pub_key.subject_public_key.to_vec()
    } else {
        let priv_key = Document::from_der(data).map_err(|_| Error::UnknownKeyType)?;
        let priv_key: PrivateKeyInfo<'_> =
            priv_key.decode_msg().map_err(|_| Error::UnknownKeyType)?;

        priv_key.private_key.to_vec()
    };

    Ok(der_key)
}

/// Decode a PEM-encoded key (PKCS#1 or PKCS#8) to PKCS#1 DER
pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
    let pem = pem::parse(data)?;

    match pem.tag.as_str() {
        "PRIVATE KEY" => pkcs8_to_pkcs1(pem.contents.as_slice(), false),
        "PUBLIC KEY" => pkcs8_to_pkcs1(pem.contents.as_slice(), true),
        "RSA PRIVATE KEY" | "RSA PUBLIC KEY" => Ok(pem.contents),
        _ => Err(Error::UnknownKeyType),
    }
}
