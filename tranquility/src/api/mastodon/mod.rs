use {
    crate::{
        consts::cors::API_ALLOWED_METHODS,
        database::{Actor, OAuthToken},
        error::Error,
        map_err,
        util::construct_cors,
    },
    headers::authorization::{Bearer, Credentials},
    once_cell::sync::Lazy,
    serde::de::DeserializeOwned,
    tranquility_types::mastodon::App,
    warp::{
        hyper::header::{HeaderValue, AUTHORIZATION},
        reject::MissingHeader,
        Filter, Rejection, Reply,
    },
};

static DEFAULT_APPLICATION: Lazy<App> = Lazy::new(|| App {
    name: "Web".into(),
    ..App::default()
});

/// Filter that can decode the body as an URL-encoded or an JSON-encoded form
pub fn urlencoded_or_json<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy
where
    T: DeserializeOwned + Send,
{
    let urlencoded_filter = warp::body::form();
    let json_filter = warp::body::json();

    urlencoded_filter.or(json_filter).unify()
}

/// Same as [`authorise_user`] but makes the bearer token optional
pub fn authorisation_optional() -> impl Filter<Extract = (Option<Actor>,), Error = Rejection> + Clone
{
    let or_none_fn = |error: Rejection| async move {
        if error.find::<MissingHeader>().is_some() {
            Ok((None,))
        } else {
            Err(error)
        }
    };

    authorisation_required().map(Some).or_else(or_none_fn)
}

/// Parses a `HeaderValue` as the contents of an authorization header with bearer token contents
/// and attempts to fetch the user from the database
async fn authorise_user(authorization_header: HeaderValue) -> Result<Actor, Rejection> {
    let credentials = Bearer::decode(&authorization_header).ok_or(Error::Unauthorized)?;
    let token = credentials.token();

    let state = crate::state::get();
    let access_token = map_err!(OAuthToken::by_access_token(&state.db_pool, token).await)?;
    let actor = map_err!(Actor::get(&state.db_pool, access_token.actor_id).await)?;

    Ok(actor)
}

/// Filter that gets the user associated with the bearer token from the database
///
/// Rejects if the user cannot be found
pub fn authorisation_required() -> impl Filter<Extract = (Actor,), Error = Rejection> + Clone {
    warp::header::value(AUTHORIZATION.as_ref()).and_then(authorise_user)
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Enable CORS for all API endpoints
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb
    let cors = construct_cors(API_ALLOWED_METHODS);

    let v1_prefix = warp::path!("api" / "v1" / ..);

    let accounts = accounts::routes();
    let apps = apps::routes();
    let statuses = statuses::routes();
    let instance = instance::routes();

    let v1_routes = accounts.or(apps).or(statuses).or(instance);

    v1_prefix.and(v1_routes).with(cors)
}

pub mod accounts;
pub mod apps;
pub mod convert;
pub mod instance;
pub mod statuses;
