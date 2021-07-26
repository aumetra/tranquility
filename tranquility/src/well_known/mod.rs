use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let routes = nodeinfo::routes().or(webfinger::routes());

    warp::path!(".well-known" / ..).and(routes)
}

pub mod nodeinfo;
pub mod webfinger;
