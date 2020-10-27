use {
    crate::error::Error,
    warp::{Filter, Rejection, Reply},
};

fn header_requirements() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header("accept")
        .and_then(|accept_header_value: String| async move {
            if accept_header_value.contains("application/activity+json")
                || accept_header_value.contains("application/ld+json")
            {
                Ok(())
            } else {
                Err(Rejection::from(Error::InvalidRequest))
            }
        })
        .untuple_one()
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let inbox = warp::path!("users" / String / "inbox")
        .and(warp::post())
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::raw())
        .and(warp::header::headers_cloned())
        .and(warp::body::json())
        .and_then(inbox::verify_request)
        .and_then(inbox::inbox);

    let objects = warp::path!("objects" / String)
        .and(warp::get())
        .and(header_requirements())
        .and_then(objects::objects);

    let users = warp::path!("users" / String)
        .and(warp::get())
        .and(header_requirements())
        .and_then(users::users);

    inbox.or(users).or(objects)
}

pub mod inbox;
pub mod objects;
pub mod users;
