use {
    crate::error::Error,
    serde::Deserialize,
    uuid::Uuid,
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

const ACTIVITY_COUNT_PER_PAGE: i64 = 10;

#[derive(Deserialize)]
pub struct CollectionQuery {
    offset: Option<u64>,
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let following = warp::path!("users" / Uuid / "following")
        .and(warp::get())
        .and(warp::query())
        .and(header_requirements())
        .and_then(following::following);

    let inbox = warp::path!("users" / Uuid / "inbox")
        .and(warp::post())
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::raw())
        .and(warp::header::headers_cloned())
        .and(warp::body::json())
        .and_then(inbox::verify_request)
        .and_then(inbox::inbox);

    let objects = warp::path!("objects" / Uuid)
        .and(warp::get())
        .and(header_requirements())
        .and_then(objects::objects);

    let outbox = warp::path!("users" / Uuid / "outbox")
        .and(warp::get())
        .and(warp::query())
        .and(header_requirements())
        .and_then(outbox::outbox);

    let users = warp::path!("users" / Uuid)
        .and(warp::get())
        .and(header_requirements())
        .and_then(users::users);

    following.or(inbox).or(objects).or(outbox).or(users)
}

pub mod following;
pub mod inbox;
pub mod objects;
pub mod outbox;
pub mod users;
