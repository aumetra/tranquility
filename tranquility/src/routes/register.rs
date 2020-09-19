use rand::Rng;
use serde::Deserialize;
use warp::Rejection;

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

pub async fn register(mut form: RegisterForm) -> Result<&'static str, Rejection> {
    form.password = crate::hashing::hash_password(form.password).await?;

    Ok("Registration endpoint")
}
