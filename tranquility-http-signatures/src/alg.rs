use {
    crate::error::{Error, Result},
    ring::{
        rand,
        signature::{RsaKeyPair, UnparsedPublicKey},
    },
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

pub struct Algorithm<'a> {
    /// Cryptographic algorithm being used (eg. `rsa`, `ecdsa`, `hmac` etc.)
    crypto: &'a str,

    /// Hash algorithm being used (eg. `sha256`, `sha384`, etc.)
    hash: &'a str,
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
    pub fn parse(raw_str: Option<&'a str>) -> Result<Self> {
        if let Some(raw_str) = raw_str {
            let (crypto, hash) =
                raw_str.split_at(raw_str.find('-').ok_or(Error::InvalidAlgorithm)?);

            // Skip the first character of the hash specifier (it's the `-`)
            let hash = &hash[1..];

            Ok((crypto, hash).into())
        } else {
            // Default to RSA-SHA256
            Ok(("rsa", "sha256").into())
        }
    }

    /// Prepare the public key for verification usage
    pub fn prepare_public_key<K>(&self, key_bytes: K) -> Result<UnparsedPublicKey<K>>
    where
        K: AsRef<[u8]>,
    {
        #[cfg(not(test))]
        let algorithm = match (self.crypto, self.hash) {
            ("rsa", "sha256") => &RSA_PKCS1_2048_8192_SHA256,
            ("rsa", "sha384") => &RSA_PKCS1_2048_8192_SHA384,
            ("rsa", "sha512") => &RSA_PKCS1_2048_8192_SHA512,

            _ => return Err(Error::UnknownKeyType),
        };

        #[cfg(test)]
        let algorithm = match (self.crypto, self.hash) {
            ("rsa", "sha256") => &RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY,
            ("rsa", "sha512") => &RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,

            _ => unreachable!(),
        };

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
