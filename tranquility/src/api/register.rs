use {
    crate::{activitypub, error::Error},
    once_cell::sync::Lazy,
    regex::Regex,
    serde::Deserialize,
    uuid::Uuid,
    validator::Validate,
    warp::{http::StatusCode, Rejection, Reply},
};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[[:alnum:]\-_]+$"#).unwrap());

#[derive(Deserialize, Validate)]
pub struct RegistrationForm {
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
    #[validate(length(min = 8))]
    password: String,
}

pub async fn register(form: RegistrationForm) -> Result<impl Reply, Rejection> {
    form.validate().map_err(Error::from)?;

    let user_id = Uuid::new_v4();
    let password_hash = crate::crypto::password::hash(form.password).await?;

    let rsa_private_key = crate::crypto::rsa::generate().await?;
    let (public_key_pem, private_key_pem) = crate::crypto::rsa::to_pem(&rsa_private_key)?;

    let actor = activitypub::create_actor(
        &user_id.to_hyphenated_ref().to_string(),
        &form.username,
        public_key_pem,
    );

    crate::database::actor::insert::local(
        user_id,
        actor,
        form.email,
        password_hash,
        private_key_pem,
    )
    .await?;

    Ok(warp::reply::with_status(
        "Account created",
        StatusCode::CREATED,
    ))
}
