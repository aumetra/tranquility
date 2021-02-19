use {
    crate::consts::USER_AGENT, futures_util::FutureExt, once_cell::sync::Lazy, reqwest::Client,
    std::future::Future, tokio::sync::oneshot, warp::cors::Cors,
};

pub static REQWEST_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().user_agent(USER_AGENT).build().unwrap());

pub fn construct_cors(allowed_methods: &[&str]) -> Cors {
    warp::cors()
        .allow_any_origin()
        .allow_methods(allowed_methods.iter().copied())
        .build()
}

/// Run any CPU intensive work (RSA key generation, password hashing, etc.) via this function
pub fn cpu_intensive_work<F, T>(func: F) -> impl Future<Output = T> + Send + Sync + 'static
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (sender, receiver) = oneshot::channel();

    rayon::spawn(move || {
        let result = func();

        if sender.send(result).is_err() {
            warn!("Couldn't send result from threadpool");
        }
    });

    receiver.map(Result::unwrap)
}

/// Format UUIDs in a unified way
#[macro_export]
macro_rules! format_uuid {
    ($uuid:expr) => {{
        $uuid.to_simple_ref().to_string()
    }};
}
