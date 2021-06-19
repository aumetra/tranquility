use {
    crate::state::ArcState,
    warp::{Filter, Rejection, Reply},
};

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let routes = nodeinfo::routes(state).or(webfinger::routes(state));

    warp::path!(".well-known" / ..).and(routes)
}

pub mod nodeinfo;
pub mod webfinger;
