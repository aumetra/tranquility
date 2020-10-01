pub mod rsa {
    use crate::error::Error;
    use openssl::{pkey::Private, rsa::Rsa};

    const KEY_SIZE: u32 = 2048;

    pub async fn generate() -> Result<Rsa<Private>, Error> {
        tokio::task::spawn_blocking(|| Ok(Rsa::generate(KEY_SIZE)?))
            .await
            .unwrap()
    }

    pub fn to_pem(rsa_key: Rsa<Private>) -> Result<(String, String), Error> {
        let public_key = String::from_utf8(rsa_key.public_key_to_pem()?).unwrap();
        let private_key = String::from_utf8(rsa_key.private_key_to_pem()?).unwrap();

        Ok((public_key, private_key))
    }
}

pub mod parse {
    use crate::error::Error;
    use openssl::pkey::{PKey, Private, Public};

    pub fn private(key_bytes: &[u8]) -> Result<PKey<Private>, Error> {
        Ok(PKey::private_key_from_pem(key_bytes)?)
    }

    pub fn public(key_bytes: &[u8]) -> Result<PKey<Public>, Error> {
        Ok(PKey::public_key_from_pem(key_bytes)?)
    }
}

pub mod password {
    use crate::error::Error;
    use argon2::Config;

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
