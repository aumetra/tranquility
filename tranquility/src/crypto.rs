pub mod rsa {
    use crate::error::Error;
    use rand::rngs::OsRng;
    use rsa::{PrivateKeyPemEncoding, PublicKeyPemEncoding, RSAPrivateKey, RSAPublicKey};

    const KEY_SIZE: usize = 2048;

    pub async fn generate() -> Result<(RSAPublicKey, RSAPrivateKey), Error> {
        tokio::task::spawn_blocking(|| {
            let private_key = RSAPrivateKey::new(&mut OsRng, KEY_SIZE)?;
            let public_key = RSAPublicKey::from(&private_key);

            Ok((public_key, private_key))
        })
        .await
        .unwrap()
    }

    pub fn to_pem(
        public_key: RSAPublicKey,
        private_key: RSAPrivateKey,
    ) -> Result<(String, String), Error> {
        let public_key = public_key.to_pem_pkcs8()?;
        let private_key = private_key.to_pem_pkcs8()?;

        Ok((public_key, private_key))
    }
}

pub mod password {
    use crate::error::Error;
    use argon2::Config;
    use rand::Rng;

    pub async fn hash(password: String) -> Result<String, Error> {
        tokio::task::spawn_blocking(move || {
            let salt = rand::thread_rng().gen::<[u8; 32]>();

            argon2::hash_encoded(password.as_bytes(), &salt, &Config::default())
        })
        .await
        .unwrap()
        .map_err(|err| err.into())
    }

    pub async fn verify(password: String, hash: String) -> bool {
        tokio::task::spawn_blocking(move || {
            argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
        })
        .await
        .unwrap()
    }
}
