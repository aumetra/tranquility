use crate::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;
use warp::{http::StatusCode, Rejection, Reply};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[[:alnum:]\-_]+$"#).unwrap());

#[derive(Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(
        length(
            min = 1,
            max = 32,
            message = "Username has to be between 1 and 32 characters long"
        ),
        regex(
            path = "USERNAME_REGEX",
            message = "Username has to consist of [A-Z, a-z, 0-9, _, -]"
        )
    )]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn register(form: RegisterForm) -> Result<impl Reply, Rejection> {
    // Validate the inputs
    form.validate().map_err(|err| Error::from(err))?;

    crate::database::actor::insert::local(form.username, form.email, form.password).await?;

    Ok(warp::reply::with_status(
        "Account created",
        StatusCode::CREATED,
    ))
}
