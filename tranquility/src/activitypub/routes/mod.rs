use warp::{Filter, Rejection, Reply};

fn header_requirements() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::exact_ignore_case("accept", "application/activity+json")
        .or(warp::header::exact_ignore_case(
            "accept",
            "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
        ))
        .unify()
        .or(warp::header::exact_ignore_case(
            "accept",
            "application/ld+json",
        ))
        .unify()
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
