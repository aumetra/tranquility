use crate::error::{Error, Result};
use ring::{
    rand,
    signature::{RsaKeyPair, UnparsedPublicKey, VerificationAlgorithm},
};

#[cfg(not(test))]
use ring::signature::{
    RSA_PKCS1_2048_8192_SHA256, RSA_PKCS1_2048_8192_SHA384, RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_SHA256, RSA_PKCS1_SHA384, RSA_PKCS1_SHA512,
};

#[cfg(test)]
use ring::signature::{
    RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY, RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_SHA256, RSA_PKCS1_SHA384, RSA_PKCS1_SHA512,
};

/// Struct holding information about the algorithms used to create the signature
///
/// Defaults to `rsa-sha256`
pub struct Algorithm<'a> {
    /// Cryptographic algorithm being used (eg. `rsa`, `ecdsa`, `hmac` etc.)
    crypto: &'a str,

    /// Hash algorithm being used (eg. `sha256`, `sha384`, etc.)
    hash: &'a str,
}

impl<'a> Default for Algorithm<'a> {
    fn default() -> Self {
        ("rsa", "sha256").into()
    }
}

impl<'a> From<(&'a str, &'a str)> for Algorithm<'a> {
    fn from((crypto, hash): (&'a str, &'a str)) -> Self {
        Self { crypto, hash }
    }
}

impl<'a> ToString for Algorithm<'a> {
    fn to_string(&self) -> String {
        format!("{}-{}", self.crypto, self.hash)
    }
}

impl<'a> Algorithm<'a> {
    /// Parse the value of the algorithm field of the signature header
    ///
    /// `Option::None` will return the default algorithm
    pub fn parse(raw_str: Option<&'a str>) -> Result<Self> {
        if let Some(raw_str) = raw_str {
            let (crypto, hash) =
                raw_str.split_at(raw_str.find('-').ok_or(Error::InvalidAlgorithm)?);

            // Skip the first character of the hash specifier (it's the `-`)
            let hash = &hash[1..];

            Ok((crypto, hash).into())
        } else {
            // Assume that the default algorithm applies
            Ok(Self::default())
        }
    }

    /// Prepare the public key for verification usage
    pub fn prepare_public_key<K>(&self, key_bytes: K) -> Result<UnparsedPublicKey<K>>
    where
        K: AsRef<[u8]>,
    {
        let algorithm = get_algorithm(self.crypto, self.hash)?;

        Ok(UnparsedPublicKey::new(algorithm, key_bytes))
    }

    /// Sign the provided data with the algorithm
    pub fn sign<K, D>(&self, key_bytes: K, data: D) -> Result<Vec<u8>>
    where
        K: AsRef<[u8]>,
        D: AsRef<[u8]>,
    {
        if self.crypto == "rsa" {
            let algorithm = match self.hash {
                "sha256" => &RSA_PKCS1_SHA256,
                "sha384" => &RSA_PKCS1_SHA384,
                "sha512" => &RSA_PKCS1_SHA512,

                _ => return Err(Error::UnknownAlgorithm),
            };

            let key_pair = RsaKeyPair::from_der(key_bytes.as_ref())?;
            let rng = rand::SystemRandom::new();

            let mut signature = vec![0; key_pair.public_modulus_len()];
            key_pair.sign(algorithm, &rng, data.as_ref(), &mut signature)?;

            Ok(signature)
        } else {
            Err(Error::UnknownAlgorithm)
        }
    }
}

/// Get the ring algorithm from the HTTP signatures crypto and hash algorithm identifier
fn get_algorithm(crypto: &str, hash: &str) -> Result<&'static dyn VerificationAlgorithm> {
    #[cfg(not(test))]
    let algorithm = match (crypto, hash) {
        ("rsa", "sha256") => &RSA_PKCS1_2048_8192_SHA256,
        ("rsa", "sha384") => &RSA_PKCS1_2048_8192_SHA384,
        ("rsa", "sha512") => &RSA_PKCS1_2048_8192_SHA512,

        _ => return Err(Error::UnknownKeyType),
    };

    // Enable unsecure key lengths for the tests because the official RFC examples are created using an RSA-1024 bit key
    #[cfg(test)]
    let algorithm = match (crypto, hash) {
        ("rsa", "sha256") => &RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY,
        ("rsa", "sha512") => &RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,

        _ => unreachable!(),
    };

    Ok(algorithm)
}
