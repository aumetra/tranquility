use {
    crate::error::Error,
    serde::{de::DeserializeOwned, Deserialize},
    uuid::Uuid,
    warp::{hyper::body::Bytes, Filter, Rejection, Reply},
};

// I wish I could use "warp::header::exact()" or something like it but the "Accept" header
// of, for example, Mastodon's fetcher look like "application/activity+json, application/ld+json".
// Because that can change for every implementation I'll just use ".contains()" on the header value
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

// The standard "warp::body::json()" filter only decodes content from requests
// that have the header "Content-Type: application/json" but the inbox
// requests have the types of either "application/ld+json" or "application/activity+json"
fn custom_json_type<T: DeserializeOwned>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy
{
    struct Json;

    impl Json {
        pub fn decode<T: DeserializeOwned>(body: &Bytes) -> Result<T, Error> {
            serde_json::from_slice(body).map_err(Error::from)
        }
    }

    warp::body::bytes().and_then(|body| async move { Ok::<T, Rejection>(Json::decode(&body)?) })
}

fn optional_raw_query() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::raw().or_else(|_| async { Ok::<_, Rejection>((String::new(),)) })
}

const ACTIVITIES_PER_PAGE: i64 = 10;

#[derive(Deserialize)]
pub struct CollectionQuery {
    offset: Option<u64>,
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let followers = warp::path!("users" / Uuid / "followers")
        .and(warp::get())
        .and(warp::query())
        .and(header_requirements())
        .and_then(followers::followers);

    let following = warp::path!("users" / Uuid / "following")
        .and(warp::get())
        .and(warp::query())
        .and(header_requirements())
        .and_then(following::following);

    let inbox = warp::path!("users" / Uuid / "inbox")
        .and(warp::post())
        .and(warp::method())
        .and(warp::path::full())
        .and(optional_raw_query())
        .and(warp::header::headers_cloned())
        .and(custom_json_type())
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

    followers
        .or(following)
        .or(inbox)
        .or(objects)
        .or(outbox)
        .or(users)
}

pub mod followers;
pub mod following;
pub mod inbox;
pub mod objects;
pub mod outbox;
pub mod users;
