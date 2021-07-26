use {
    crate::{database::Actor, error::Error as OtherError, map_err, state::State},
    arc_swap::{ArcSwap, Guard},
    lettre::{
        error::Error as ContentError,
        transport::smtp::{authentication::Credentials, Error as SmtpError},
        AsyncTransport, Message, Tokio1Executor,
    },
    once_cell::sync::OnceCell,
    ormx::Table,
    std::sync::Arc,
    warp::{Filter, Rejection, Reply},
};

type AsyncSmtpTransport = lettre::AsyncSmtpTransport<Tokio1Executor>;

static SMTP_TRANSPORT: OnceCell<ArcSwap<AsyncSmtpTransport>> = OnceCell::new();

#[derive(Debug, thiserror::Error)]
/// Email-related errors
enum Error {
    #[error("Content error: {0}")]
    Content(#[from] ContentError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Other error: {0}")]
    Other(#[from] OtherError),

    #[error("SMTP transport error: {0}")]
    Smtp(#[from] SmtpError),
}

#[inline]
/// Initialise the SMTP transport
fn init_transport(state: &State) -> Result<Arc<AsyncSmtpTransport>, Error> {
    let transport_builder = if state.config.email.starttls {
        AsyncSmtpTransport::relay(&state.config.email.server)
    } else {
        AsyncSmtpTransport::starttls_relay(&state.config.email.server)
    }?;

    let username = state.config.email.username.to_string();
    let password = state.config.email.password.to_string();
    let transport = transport_builder
        .credentials(Credentials::new(username, password))
        .build();

    Ok(Arc::new(transport))
}

/// Get a reference to the global SMTP transport (or initialise one if there isn't one already)
fn get_transport(state: &State) -> Result<Guard<Arc<AsyncSmtpTransport>>, Error> {
    SMTP_TRANSPORT
        .get_or_try_init::<_, Error>(|| {
            let transport = init_transport(state)?;

            Ok(ArcSwap::new(transport))
        })
        .map(ArcSwap::load)
}

/// Attempt to update the transport on configuration change
pub fn update_transport() {
    let state = crate::state::get();
    let transport = match init_transport(&state) {
        Ok(transport) => transport,
        Err(err) => {
            warn!(error = ?err, "Failed to construct SMTP transport");
            return;
        }
    };

    if let Some(global_transport) = SMTP_TRANSPORT.get() {
        global_transport.swap(transport);
    }
}

pub fn send_confirmation(mut user: Actor) {
    let state = crate::state::get_full();

    if !state.config.email.active {
        return;
    }

    // Spawn off here since we don't want to delay the request processing
    tokio::spawn(async move {
        // Run the actual logic inside an own async block to be able to take advantage of
        // the try syntax and to handle all the errors in a single place
        let result: Result<(), Error> = async move {
            // Generate and save the confirmation code
            let confirmation_code = crate::crypto::token::generate()?;
            user.confirmation_code = Some(confirmation_code.clone());
            user.is_confirmed = false;
            user.update(&state.db_pool).await?;

            let domain = &state.config.instance.domain;
            let confirmation_url = format!("https://{}/confirm-account/{}", domain, confirmation_code);
            let message_body = format!(
                "Hello, thank you for creating an account on {}!\nTo confirm your account, please visit the URL below:\n{}", 
                domain,
                confirmation_url
            );

            let from_mailbox = state.config.email.email.parse().unwrap();
            let to_mailbox = user.email.unwrap().parse().unwrap();
            let message = Message::builder().subject("Account confirmation").from(from_mailbox).to(to_mailbox).body(message_body)?;

            let transport = get_transport(&state)?;
            transport.send(message).await?;

            Ok(())
        }
        .await;

        if let Err(err) = result {
            error!(error = ?err, "Couldn't send confirmation email")
        }
    });
}

async fn confirm_account(confirmation_code: String) -> Result<impl Reply, Rejection> {
    let state = crate::state::get();
    let mut user = map_err!(Actor::by_confirmation_code(&state.db_pool, &confirmation_code).await)?;
    user.is_confirmed = true;
    map_err!(user.update(&state.db_pool).await)?;

    Ok("Account confirmed!")
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("confirm-account" / String).and_then(confirm_account)
}
