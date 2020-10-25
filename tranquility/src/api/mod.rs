use {
    serde::de::DeserializeOwned,
    warp::{Filter, Rejection, Reply},
};

pub fn form_urlencoded_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::form().or(warp::body::json()).unify()
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(register::register);

    register
}

pub mod register;
