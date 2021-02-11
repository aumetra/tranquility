use {
    futures_util::FutureExt, once_cell::sync::Lazy, reqwest::Client, std::future::Future,
    tokio::sync::oneshot,
};

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("GIT_BRANCH"),
    "-",
    env!("GIT_COMMIT")
);

pub static REQWEST_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().user_agent(USER_AGENT).build().unwrap());

/// Run CPU intensive work (RSA key generation, password hashing, etc.) via this function
pub fn cpu_intensive_work<T>(
    func: impl FnOnce() -> T + Send + 'static,
) -> impl Future<Output = T> + Send + Sync + 'static
where
    T: Send + 'static,
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
