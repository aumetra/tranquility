use rand::{
    distributions::{Distribution, Standard},
    rngs::OsRng,
    Rng,
};

/// Generate a type that supports being generated via `rand::Rng::gen()` using `OsRng`
pub fn gen_secure_rand<T>() -> T
where
    Standard: Distribution<T>,
{
    OsRng.gen::<T>()
}

pub mod digest {
    use {
        crate::util::cpu_intensive_task,
        reqwest::header::HeaderValue,
        sha2::{Digest, Sha256},
    };

    /// Calculate the digest HTTP header
    pub async fn http_header(data: Vec<u8>) -> anyhow::Result<HeaderValue> {
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
    use {crate::util::cpu_intensive_task, argon2::Config};

    /// Hash the password using the standard rust-argon2 config
    pub async fn hash(password: String) -> anyhow::Result<String> {
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
    use {
        crate::{consts::crypto::KEY_SIZE, util::cpu_intensive_task},
        rand::rngs::OsRng,
        rsa::{PrivateKeyPemEncoding, PublicKeyPemEncoding, RSAPrivateKey},
    };

    /// Generate an RSA key pair (key size defined in the `consts` file)
    pub async fn generate() -> anyhow::Result<RSAPrivateKey> {
        cpu_intensive_task(|| Ok(RSAPrivateKey::new(&mut OsRng, KEY_SIZE)?)).await
    }

    /// Get the public key from the private key and encode both in the PKCS#8 PEM format
    pub fn to_pem(rsa_key: &RSAPrivateKey) -> anyhow::Result<(String, String)> {
        let public_key = PublicKeyPemEncoding::to_pem_pkcs8(&rsa_key.to_public_key())?;
        let private_key = PrivateKeyPemEncoding::to_pem_pkcs8(rsa_key)?;

        Ok((public_key, private_key))
    }
}

pub mod token {
    use crate::{consts::crypto::TOKEN_LENGTH, error::Error};

    /// Generate a cryptographically random token (length defined in the `consts` file)
    pub fn generate() -> Result<String, Error> {
        // Two characters are needed to encode one byte as hex
        let token = crate::crypto::gen_secure_rand::<[u8; TOKEN_LENGTH / 2]>();

        Ok(hex::encode(token))
    }
}

pub mod request {
    use {
        crate::util::cpu_intensive_task,
        std::future::Future,
        tranquility_http_signatures::Request,
        warp::{
            http::{
                header::{HeaderMap, HeaderName, HeaderValue},
                Method,
            },
            path::FullPath,
        },
    };

    /// Sign a reqwest HTTP request
    pub fn sign(
        request: reqwest::Request,
        key_id: String,
        // The public key is provided in the PEM format
        // That's why the function takes a `String`
        private_key: String,
    ) -> impl Future<Output = anyhow::Result<(HeaderName, HeaderValue)>> + Send {
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
    pub fn verify(
        method: Method,
        path: FullPath,
        query: Option<String>,
        headers: HeaderMap,
        // The public key is provided in the PEM format
        // That's why the function takes a `String`
        public_key: String,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send {
        cpu_intensive_task(move || {
            let method = method.as_str();
            let path = path.as_str();
            let query = query.as_deref();
            let headers = &headers;
            let public_key = public_key.as_bytes();

            let request = Request::new(method, path, query, headers);

            Ok(tranquility_http_signatures::verify(request, public_key)?)
        })
    }
}
