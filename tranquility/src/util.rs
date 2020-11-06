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
