use {
    crate::{database::Actor, error::Error as OtherError, state::ArcState},
    lettre::{
        error::Error as ContentError,
        transport::smtp::{authentication::Credentials, Error as SmtpError},
        Message, Tokio1Executor,
    },
    once_cell::sync::OnceCell,
    std::sync::Arc,
};

type AsyncSmtpTransport = lettre::AsyncSmtpTransport<Tokio1Executor>;

static SMTP_TRANSPORT: OnceCell<AsyncSmtpTransport> = OnceCell::new();

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Content error: {0}")]
    Content(#[from] ContentError),

    #[error("Other error: {0}")]
    Other(#[from] OtherError),

    #[error("SMTP transport error: {0}")]
    Smtp(#[from] SmtpError),
}

#[inline]
/// Initialise the SMTP transport
fn init_transport(state: &ArcState) -> Result<AsyncSmtpTransport, Error> {
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

    Ok(transport)
}

#[inline]
/// Get a reference to the global SMTP transport (or initialise one if there isn't one already)
fn get_transport(state: &ArcState) -> Result<&'static AsyncSmtpTransport, Error> {
    SMTP_TRANSPORT.get_or_try_init::<_, Error>(|| init_transport(&state))
}

pub fn send_confirmation(state: &ArcState, user: Actor) {
    let state = Arc::clone(&state);

    // Spawn off here since we don't want to delay the request processing
    tokio::spawn(async move {
        // Run the actual logic inside an own async block to be able to take advantage of
        // the try syntax and to handle all the errors in a single place
        let result: Result<(), Error> = async move {
            let transport = get_transport(&state)?;

            // TODO: Generate confimation code, insert code into database, create endpoint for confirming (with ratelimit ofc)

            Ok(())
        }
        .await;

        if let Err(err) = result {
            error!(error = ?err, "Couldn't send confirmation email")
        }
    });
}
