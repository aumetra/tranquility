use {
    crate::{
        activitypub,
        consts::regex::USERNAME,
        database::{InsertActor, InsertExt},
        limit_body_size, map_err, regex,
    },
    serde::Deserialize,
    tranquility_ratelimit::{ratelimit, Configuration},
    uuid::Uuid,
    validator::Validate,
    warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply},
};

regex!(USERNAME_REGEX = USERNAME);

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
    #[validate(length(min = 8, message = "Password has to be at least 8 characters long"))]
    password: String,
}

async fn register(form: RegistrationForm) -> Result<Response, Rejection> {
    let state = crate::state::get();
    if state.config.instance.closed_registrations {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    map_err!(form.validate())?;

    let user_id = Uuid::new_v4();
    let password_hash = crate::crypto::password::hash(form.password).await?;

    let rsa_private_key = crate::crypto::rsa::generate().await?;
    let (public_key_pem, private_key_pem) = crate::crypto::rsa::to_pem(&rsa_private_key)?;

    let actor = activitypub::instantiate::actor(
        &state.config,
        &user_id.to_hyphenated_ref().to_string(),
        &form.username,
        public_key_pem,
    );
    let actor = map_err!(serde_json::to_value(&actor))?;

    let _user = InsertActor {
        id: user_id,
        username: form.username,
        actor,
        email: Some(form.email),
        password_hash: Some(password_hash),
        private_key: Some(private_key_pem),
        remote: false,

        is_confirmed: true,
        confirmation_code: None,
    }
    .insert(&state.db_pool)
    .await?;

    #[cfg(feature = "email")]
    crate::email::send_confirmation(_user);

    Ok(warp::reply::with_status("Account created", StatusCode::CREATED).into_response())
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state = crate::state::get();
    let active = state.config.ratelimit.active;
    let registration_quota = state.config.ratelimit.registration_quota;

    let ratelimit_config = Configuration::new()
        .active(active)
        .burst_quota(registration_quota);

    // Ratelimit only the logic
    let ratelimit_wrapper =
        ratelimit!(from_config: ratelimit_config).expect("Couldn't construct ratelimit wrapper");
    let register_logic = warp::post()
        .and(warp::body::form())
        .and_then(register)
        .with(ratelimit_wrapper);
    // Restrict the body size
    let register_logic = limit_body_size!(register_logic);

    warp::path!("api" / "tranquility" / "v1" / "register").and(register_logic)
}
