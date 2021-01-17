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
        crate::{cpu_intensive_work, error::Error},
        reqwest::header::HeaderValue,
        sha2::{Digest, Sha256},
    };

    pub async fn http_header(data: Vec<u8>) -> Result<HeaderValue, Error> {
        cpu_intensive_work!(move || {
            let sha_hash = Sha256::digest(&data);
            let base64_encoded_hash = base64::encode(&sha_hash);

            Ok(HeaderValue::from_str(&format!(
                "SHA-256={}",
                base64_encoded_hash
            ))?)
        })
        .await
        .unwrap()
    }
}

pub mod password {
    use {
        crate::{cpu_intensive_work, error::Error},
        argon2::Config,
    };

    pub async fn hash(password: String) -> Result<String, Error> {
        cpu_intensive_work!(move || {
            let salt = crate::crypto::gen_secure_rand::<[u8; 32]>();

            Ok(argon2::hash_encoded(
                password.as_bytes(),
                &salt,
                &Config::default(),
            )?)
        })
        .await
        .unwrap()
    }

    pub async fn verify(password: String, hash: String) -> bool {
        cpu_intensive_work!(move || {
            argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
        })
        .await
        .unwrap()
    }
}

pub mod rsa {
    use {
        crate::{cpu_intensive_work, error::Error},
        rand::rngs::OsRng,
        rsa::{PrivateKeyPemEncoding, PublicKeyPemEncoding, RSAPrivateKey},
    };

    const KEY_SIZE: usize = 2048;

    pub async fn generate() -> Result<RSAPrivateKey, Error> {
        cpu_intensive_work!(|| Ok(RSAPrivateKey::new(&mut OsRng, KEY_SIZE)?))
            .await
            .unwrap()
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
