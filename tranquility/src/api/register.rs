use crate::{
    activitypub,
    consts::{regex::USERNAME, MAX_BODY_SIZE},
    database::{InsertActor, InsertExt},
    error::Error,
    format_uuid, ratelimit_layer, regex,
    state::{ArcState, State},
    util::Form,
};
use axum::{
    extract::ContentLengthLimit, http::StatusCode, response::IntoResponse, routing::post,
    Extension, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

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

async fn register(
    Extension(state): Extension<ArcState>,
    ContentLengthLimit(Form(form)): ContentLengthLimit<Form<RegistrationForm>, MAX_BODY_SIZE>,
) -> Result<impl IntoResponse, Error> {
    if state.config.instance.closed_registrations {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    form.validate()?;

    let user_id = Uuid::new_v4();
    let password_hash = crate::crypto::password::hash(form.password).await?;

    let rsa_private_key = crate::crypto::rsa::generate().await?;
    let (public_key_pem, private_key_pem) = crate::crypto::rsa::to_pem(&rsa_private_key)?;

    let actor = activitypub::instantiate::actor(
        &state.config,
        &format_uuid!(user_id),
        &form.username,
        public_key_pem,
    );
    let actor = serde_json::to_value(&actor)?;

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
    crate::email::send_confirmation(&state, _user);

    Ok((StatusCode::CREATED, "Account created").into_response())
}

pub fn routes(state: &State) -> Router {
    Router::new()
        .route("/api/tranquility/v1/register", post(register))
        .route_layer(ratelimit_layer!(
            state.config.ratelimit.active,
            state.config.ratelimit.use_forwarded_header,
            state.config.ratelimit.registration_quota,
        ))
}
