use {
    futures_util::FutureExt,
    std::future::Future,
    tokio::sync::oneshot,
    warp::{reply::Response, Reply},
};

pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Reply for Either<A, B>
where
    A: Reply,
    B: Reply,
{
    fn into_response(self) -> Response {
        match self {
            Either::A(a) => a.into_response(),
            Either::B(b) => b.into_response(),
        }
    }
}

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
