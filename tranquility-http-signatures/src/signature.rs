use crate::{
    error::Result,
    util::{HashMapExt as _, IteratorExt as _},
};
use http::header::HeaderValue;

/// Parsed form of a signature header
pub struct Signature<'a> {
    /// ID of the associated public key
    pub(crate) key_id: &'a str,

    /// Used algorithm (if empty, default to RSA-SHA256)
    pub(crate) algorithm: Option<&'a str>,

    /// Headers used by the signature
    pub(crate) headers: Vec<&'a str>,

    /// base64-encoded signature
    pub(crate) signature: &'a str,

    /// Timestamp when the signature was created
    pub(crate) created: Option<&'a str>,

    /// Timestamp when the signature will expire
    pub(crate) expires: Option<&'a str>,
}

impl<'a> Signature<'a> {
    /// Create a new signature
    pub fn new(
        key_id: &'a str,
        algorithm: Option<&'a str>,
        headers: Vec<&'a str>,
        encoded_signature: &'a str,
        created: Option<&'a str>,
        expires: Option<&'a str>,
    ) -> Self {
        Self {
            key_id,
            algorithm,
            headers,
            signature: encoded_signature,
            created,
            expires,
        }
    }

    /// Encode the signature into a `HeaderValue`
    pub fn encode(self) -> Result<HeaderValue> {
        let mut signature = format!(
            r#"keyId="{}",headers="{}",signature="{}""#,
            self.key_id,
            self.headers.join(" "),
            self.signature
        );

        if let Some(algorithm) = self.algorithm {
            append_key(&mut signature, "algorithm", algorithm);
        }

        if let Some(created) = self.created {
            append_key(&mut signature, "created", created);
        }

        if let Some(expires) = self.expires {
            append_key(&mut signature, "expires", expires);
        }

        let header_value = HeaderValue::from_str(signature.as_str())?;
        Ok(header_value)
    }

    /// Parse a raw `&str` into an `Signature`
    pub fn parse(raw_str: &'a str) -> Result<Self> {
        let parsed_header_value = raw_str
            .split(',')
            .filter_map(|kv_pair| {
                let (key, value) = kv_pair.split_at(kv_pair.find('=')?);

                // Skip the first character because the first character is the '='
                let value = &value[1..];

                // Clean up the key and value
                let key = key.trim();
                let value = value.trim_matches('"');

                Some((key, value))
            })
            .collect_hashmap();

        let key_id = parsed_header_value.get_signature_field("keyId")?;

        let algorithm = parsed_header_value.get_signature_field("algorithm").ok();

        // The header field might be absent
        let headers = parsed_header_value
            .get_signature_field("headers")
            .unwrap_or_default()
            .split_whitespace()
            .collect_vec();

        let signature = parsed_header_value.get_signature_field("signature")?;

        let created = parsed_header_value.get_signature_field("created").ok();
        let expires = parsed_header_value.get_signature_field("expires").ok();

        let signature_string = Signature {
            key_id,
            algorithm,
            headers,
            signature,
            created,
            expires,
        };

        Ok(signature_string)
    }
}

/// Append a key-value pair to the signature
fn append_key(sig: &mut String, key: &str, value: &str) {
    sig.push(',');
    sig.push_str(key);
    sig.push_str("=\"");
    sig.push_str(value);
    sig.push('"');
}
