use {
    serde::de::DeserializeOwned,
    warp::{Filter, Rejection, Reply},
};

pub fn form_urlencoded_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::form().or(warp::body::json()).unify()
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let apps = warp::path!("api" / "v1" / "apps")
        .and(warp::post())
        .and(form_urlencoded_json())
        .and_then(apps::create);

    apps
}

pub mod apps;
pub mod convert;
