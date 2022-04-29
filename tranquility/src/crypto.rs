use rand::{
    distributions::{Distribution, Standard},
    rngs::OsRng,
    Rng,
};

/// Generate a type that supports being generated via `rand::Rng::gen()` using `OsRng`
#[inline]
pub fn gen_secure_rand<T>() -> T
where
    Standard: Distribution<T>,
{
    OsRng.gen::<T>()
}

pub mod digest {
    use crate::{error::Error, util::cpu_intensive_task};
    use reqwest::header::HeaderValue;
    use sha2::{Digest, Sha256};

    /// Calculate the digest HTTP header
    pub async fn http_header(data: Vec<u8>) -> Result<HeaderValue, Error> {
        cpu_intensive_task(move || {
            let sha_hash = Sha256::digest(&data);
            let base64_encoded_hash = base64::encode(&sha_hash);

            Ok(HeaderValue::from_str(&format!(
                "SHA-256={}",
                base64_encoded_hash
            ))?)
        })
        .await
    }
}

pub mod password {
    use crate::{error::Error, util::cpu_intensive_task};
    use argon2::Config;

    /// Hash the password using the standard rust-argon2 config
    pub async fn hash(password: String) -> Result<String, Error> {
        cpu_intensive_task(move || {
            let salt = crate::crypto::gen_secure_rand::<[u8; 32]>();
            let config = Config::default();

            Ok(argon2::hash_encoded(password.as_bytes(), &salt, &config)?)
        })
        .await
    }

    /// Verify an encoded password
    pub async fn verify(password: String, hash: String) -> bool {
        cpu_intensive_task(move || {
            argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
        })
        .await
    }
}

pub mod rsa {
    use crate::{consts::crypto::KEY_SIZE, error::Error, util::cpu_intensive_task};
    use rand::rngs::OsRng;
    use rsa::{
        pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
        RsaPrivateKey,
    };

    /// Generate an RSA key pair (key size defined in the `consts` file)
    pub async fn generate() -> Result<RsaPrivateKey, Error> {
        cpu_intensive_task(|| Ok(RsaPrivateKey::new(&mut OsRng, KEY_SIZE)?)).await
    }

    /// Get the public key from the private key and encode both in the PKCS#8 PEM format
    pub fn to_pem(rsa_key: &RsaPrivateKey) -> Result<(String, String), Error> {
        let public_key = rsa_key.to_public_key_pem(LineEnding::LF)?;
        let private_key = rsa_key.to_pkcs8_pem(LineEnding::LF)?;

        Ok((public_key, private_key))
    }
}

pub mod token {
    use crate::consts::crypto::TOKEN_LENGTH;

    /// Generate a cryptographically random token (length defined in the `consts` file)
    pub fn generate() -> String {
        // Two characters are needed to encode one byte as hex
        let token = crate::crypto::gen_secure_rand::<[u8; TOKEN_LENGTH / 2]>();

        hex::encode(token)
    }
}

pub mod request {
    use crate::{error::Error, util::cpu_intensive_task};
    use axum::http::{
        self,
        header::{HeaderName, HeaderValue},
    };
    use std::future::Future;

    /// Sign a reqwest HTTP request
    pub fn sign(
        request: reqwest::Request,
        key_id: String,
        // The public key is provided in the PEM format
        // That's why the function takes a `String`
        private_key: String,
    ) -> impl Future<Output = Result<(HeaderName, HeaderValue), Error>> + Send {
        cpu_intensive_task(move || {
            let request = &request;
            let key_id = key_id.as_str();
            let private_key = private_key.as_bytes();

            Ok(tranquility_http_signatures::sign(
                request,
                &["(request-target)", "date", "digest"],
                (key_id, private_key),
            )?)
        })
    }

    /// Verify an HTTP request using parameters obtained from warp
    pub fn verify<B>(
        request: http::Request<B>,
        // The public key is provided in the PEM format
        // That's why the function takes a `String`
        public_key: String,
    ) -> impl Future<Output = Result<bool, Error>> + Send {
        cpu_intensive_task(move || {
            let public_key = public_key.as_bytes();

            Ok(tranquility_http_signatures::verify(&request, public_key)?)
        })
    }
}
