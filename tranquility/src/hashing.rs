use crate::error::Error;
use argon2::Config;
use rand::Rng;

pub async fn hash_password(password: String) -> Result<String, Error> {
    tokio::task::spawn_blocking(move || {
        let salt = rand::thread_rng().gen::<[u8; 32]>();

        argon2::hash_encoded(password.as_bytes(), &salt, &Config::default())
    })
    .await
    .unwrap()
    .map_err(|err| err.into())
}

pub async fn verify_password(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap_or(false)
    })
    .await
    .unwrap()
}
