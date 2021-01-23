use rand::{
    distributions::{Distribution, Standard},
    rngs::OsRng,
    Rng,
};

pub fn gen_secure_rand<T>() -> T
where
    Standard: Distribution<T>,
{
    OsRng.gen::<T>()
}

pub mod digest {
    use {
        crate::{error::Error, util::cpu_intensive_work},
        reqwest::header::HeaderValue,
        sha2::{Digest, Sha256},
    };

    pub async fn http_header(data: Vec<u8>) -> Result<HeaderValue, Error> {
        cpu_intensive_work(move || {
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
    use {
        crate::{error::Error, util::cpu_intensive_work},
        argon2::Config,
    };

    pub async fn hash(password: String) -> Result<String, Error> {
        cpu_intensive_work(move || {
            let salt = crate::crypto::gen_secure_rand::<[u8; 32]>();

            Ok(argon2::hash_encoded(
                password.as_bytes(),
                &salt,
                &Config::default(),
            )?)
        })
        .await
    }

    pub async fn verify(password: String, hash: String) -> bool {
        cpu_intensive_work(move || {
            argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
        })
        .await
    }
}

pub mod rsa {
    use {
        crate::{error::Error, util::cpu_intensive_work},
        rand::rngs::OsRng,
        rsa::{PrivateKeyPemEncoding, PublicKeyPemEncoding, RSAPrivateKey},
    };

    const KEY_SIZE: usize = 2048;

    pub async fn generate() -> Result<RSAPrivateKey, Error> {
        cpu_intensive_work(|| Ok(RSAPrivateKey::new(&mut OsRng, KEY_SIZE)?)).await
    }

    pub fn to_pem(rsa_key: &RSAPrivateKey) -> Result<(String, String), Error> {
        let public_key = PublicKeyPemEncoding::to_pem_pkcs8(&rsa_key.to_public_key())?;
        let private_key = PrivateKeyPemEncoding::to_pem_pkcs8(rsa_key)?;

        Ok((public_key, private_key))
    }
}

pub mod token {
    use crate::error::Error;

    const TOKEN_LENGTH: usize = 40;

    pub fn generate() -> Result<String, Error> {
        // Two characters are needed to encode one byte as hex
        let token = crate::crypto::gen_secure_rand::<[u8; TOKEN_LENGTH / 2]>();

        Ok(hex::encode(token))
    }
}

pub mod request {
    use {
        crate::{error::Error, util::cpu_intensive_work},
        reqwest::Request,
        std::future::Future,
        tranquility_http_signatures::HttpRequest,
        warp::{
            http::{
                header::{HeaderMap, HeaderName, HeaderValue},
                Method,
            },
            path::FullPath,
        },
    };

    pub fn sign(
        request: Request,
        key_id: String,
        private_key: String,
    ) -> impl Future<Output = Result<(HeaderName, HeaderValue), Error>> + Send {
        cpu_intensive_work(move || {
            let request = &request;
            let key_id = key_id.as_str();
            let private_key = private_key.as_bytes();

            tranquility_http_signatures::sign(
                request,
                key_id,
                &["(request-target)", "date", "digest"],
                private_key,
            )
            .map_err(Error::from)
        })
    }

    pub fn verify(
        method: Method,
        path: FullPath,
        query: Option<String>,
        headers: HeaderMap,
        // The public key is provided in the PEM format
        // That's why the function takes a `String`
        public_key: String,
    ) -> impl Future<Output = Result<bool, Error>> + Send {
        cpu_intensive_work(move || {
            let method = method.as_str();
            let path = path.as_str();
            let query = query.as_deref();
            let headers = &headers;
            let public_key = public_key.as_bytes();

            let request = HttpRequest::new(method, path, query, headers);

            tranquility_http_signatures::verify(request, public_key).map_err(Error::from)
        })
    }
}
