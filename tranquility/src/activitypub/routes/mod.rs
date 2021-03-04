use {
    crate::{config::ArcConfig, error::Error},
    serde::{de::DeserializeOwned, Deserialize},
    uuid::Uuid,
    warp::{hyper::body::Bytes, Filter, Rejection, Reply},
};

// I wish I could use "warp::header::exact()" or something like it but the "Accept" header.
// But the value can change for every implementation, for example, Mastodon's fetcher look like "application/activity+json, application/ld+json".
// So I'll just use ".contains()" on the header value
fn header_requirements() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    let header_requirements_fn = |accept_header_value: String| async move {
        if accept_header_value.contains("application/activity+json")
            || accept_header_value.contains("application/ld+json")
        {
            Ok(())
        } else {
            Err(Rejection::from(Error::InvalidRequest))
        }
    };

    warp::header("accept")
        .and_then(header_requirements_fn)
        .untuple_one()
}

// The standard "warp::body::json()" filter only decodes content from requests
// that have the header "Content-Type: application/json" but the inbox
// requests have the types of either "application/ld+json" or "application/activity+json"
fn custom_json_parser<T: DeserializeOwned>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy
{
    let custom_json_parser_fn = |body: Bytes| async move {
        let value = serde_json::from_slice(&body).map_err(Error::from)?;

        Ok::<T, Rejection>(value)
    };

    warp::body::bytes().and_then(custom_json_parser_fn)
}

fn optional_raw_query() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::raw().or_else(|_| async { Ok::<_, Rejection>((String::new(),)) })
}

#[derive(Deserialize)]
pub struct CollectionQuery {
    last_id: Option<Uuid>,
}

pub fn routes(
    config: ArcConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let config = crate::config::filter(config);

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
        .and(config)
        .and(inbox::validate_request())
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
