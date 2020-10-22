use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let register = warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(register::register);

    register
}

pub mod register;
