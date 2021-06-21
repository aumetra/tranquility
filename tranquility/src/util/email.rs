use {
    crate::{database::Actor, error::Error, state::ArcState},
    lettre::{transport::smtp::authentication::Credentials, Message, Tokio1Executor},
    once_cell::sync::OnceCell,
    std::sync::Arc,
};

type AsyncSmtpTransport = lettre::AsyncSmtpTransport<Tokio1Executor>;

static SMTP_TRANSPORT: OnceCell<AsyncSmtpTransport> = OnceCell::new();

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

pub fn send_confirmation(state: &ArcState, user: Actor) {
    let state = Arc::clone(&state);

    // Spawn off here since we don't want to delay the request processing
    tokio::spawn(async move {
        let transport = SMTP_TRANSPORT.get_or_try_init::<_, Error>(|| init_transport(&state));

        // TODO: Generate confimation code, insert code into database, create endpoint for confirming (with ratelimit ofc)
    });
}
