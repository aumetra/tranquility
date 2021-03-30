/// Public key for verifying the request
pub type PublicKey<'a> = &'a [u8];

/// Private key for signing the request
pub struct PrivateKey<'a> {
    /// ID of the associated public key (this is usually an URL pointing to the key)
    key_id: &'a str,

    /// Private key in PKCS#8 PEM format
    data: &'a [u8],
}

impl<'a> PrivateKey<'a> {
    #[must_use]
    /// Create a new private key
    pub fn new(key_id: &'a str, data: &'a [u8]) -> Self {
        Self { key_id, data }
    }
}

impl<'a> From<(&'a str, &'a [u8])> for PrivateKey<'a> {
    fn from((key_id, data): (&'a str, &'a [u8])) -> Self {
        Self::new(key_id, data)
    }
}
