use crate::error::Error;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use warp::{http::StatusCode, Rejection, Reply};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    let mut builder = RegexBuilder::new(r#"^[a-z0-9\-_]{1,32}$"#);
    builder.case_insensitive(true).build().unwrap()
});

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

pub async fn register(form: RegisterForm) -> Result<impl Reply, Rejection> {
    // Check if the username is valid
    if !USERNAME_REGEX.is_match(&form.username) {
        return Err(Error::InvalidUsername.into());
    }

    let password_hash = crate::crypto::password::hash(form.password).await?;
    let (rsa_public_key, rsa_private_key) = crate::crypto::rsa::generate().await?;
    let (public_key_pem, private_key_pem) =
        crate::crypto::rsa::to_pem(rsa_public_key, rsa_private_key)?;

    crate::database::actor::insert::local(
        form.username,
        form.email,
        password_hash,
        public_key_pem,
        private_key_pem,
    )
    .await?;

    Ok(warp::reply::with_status(
        "Account created",
        StatusCode::CREATED,
    ))
}
