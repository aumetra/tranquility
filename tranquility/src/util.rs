use warp::{reply::Response, Reply};

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

#[macro_export]
macro_rules! cpu_intensive_work {
    ($func:expr) => {{
        let (sender, receiver) = tokio::sync::oneshot::channel();

        rayon::spawn(move || {
            let result = $func();

            if sender.send(result).is_err() {
                tracing::warn!("Couldn't send result from threadpool");
            }
        });

        receiver
    }};
}

// Macro formatting UUIDs in a unified way
#[macro_export]
macro_rules! format_uuid {
    ($uuid:expr) => {{
        $uuid.to_simple_ref().to_string()
    }};
}
