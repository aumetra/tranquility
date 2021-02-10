use {futures_util::FutureExt, std::future::Future, tokio::sync::oneshot};

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

// Macro formatting UUIDs in a unified way
#[macro_export]
macro_rules! format_uuid {
    ($uuid:expr) => {{
        $uuid.to_simple_ref().to_string()
    }};
}
