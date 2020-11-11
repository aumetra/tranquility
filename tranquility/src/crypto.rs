pub mod digest {
    use {crate::error::Error, reqwest::header::HeaderValue};

    pub async fn http_header(data: Vec<u8>) -> Result<HeaderValue, Error> {
        tokio::task::spawn_blocking(move || {
            let sha_hash = openssl::sha::sha256(&data);
            let base64_encoded_hash = openssl::base64::encode_block(&sha_hash);

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
    use {crate::error::Error, argon2::Config};

    pub async fn hash(password: String) -> Result<String, Error> {
        tokio::task::spawn_blocking(move || {
            let mut salt = [0; 32];
            openssl::rand::rand_bytes(&mut salt)?;

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
        tokio::task::spawn_blocking(move || {
            argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
        })
        .await
        .unwrap()
    }
}

pub mod rsa {
    use {
        crate::error::Error,
        openssl::{pkey::Private, rsa::Rsa},
    };

    const KEY_SIZE: u32 = 2048;

    pub async fn generate() -> Result<Rsa<Private>, Error> {
        tokio::task::spawn_blocking(|| Ok(Rsa::generate(KEY_SIZE)?))
            .await
            .unwrap()
    }

    pub fn to_pem(rsa_key: &Rsa<Private>) -> Result<(String, String), Error> {
        let public_key = String::from_utf8(rsa_key.public_key_to_pem()?).unwrap();
        let private_key = String::from_utf8(rsa_key.private_key_to_pem()?).unwrap();

        Ok((public_key, private_key))
    }
}

pub mod token {
    use crate::error::Error;

    const TOKEN_LENGTH: usize = 40;

    pub fn generate() -> Result<String, Error> {
        // Two characters are needed to encode one byte as hex
        let mut token = [0; TOKEN_LENGTH / 2];
        openssl::rand::rand_bytes(&mut token)?;

        Ok(hex::encode(token))
    }
}
