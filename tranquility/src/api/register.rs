use {
    crate::{activitypub, map_err, state::ArcState},
    once_cell::sync::Lazy,
    regex::Regex,
    serde::Deserialize,
    tranquility_ratelimit::{ratelimit, Configuration},
    uuid::Uuid,
    validator::Validate,
    warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply},
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
    #[validate(length(min = 8, message = "Password has to be at least 8 characters long"))]
    password: String,
}

async fn register(state: ArcState, form: RegistrationForm) -> Result<Response, Rejection> {
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

    crate::database::actor::insert::local(
        &state.db_pool,
        user_id,
        actor,
        form.email,
        password_hash,
        private_key_pem,
    )
    .await?;

    Ok(warp::reply::with_status("Account created", StatusCode::CREATED).into_response())
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let active = state.config.ratelimit.active;
    let registration_quota = state.config.ratelimit.registration_quota;

    let ratelimit_config = Configuration::new()
        .active(active)
        .burst_quota(registration_quota);

    let state_filter = crate::state::filter(state);

    // Ratelimit only the logic
    let ratelimit_wrapper =
        ratelimit!(from_config: ratelimit_config).expect("Couldn't construct ratelimit wrapper");
    let register_logic = warp::post()
        .and(state_filter)
        .and(warp::body::form())
        .and_then(register)
        .with(ratelimit_wrapper);

    warp::path!("api" / "tranquility" / "v1" / "register").and(register_logic)
}
