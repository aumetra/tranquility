use {
    crate::{
        consts::cors::API_ALLOWED_METHODS,
        database::model::{Actor, OAuthToken},
        error::Error,
        state::ArcState,
        util::construct_cors,
    },
    headers::authorization::{Bearer, Credentials},
    once_cell::sync::Lazy,
    ormx::Table,
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

pub fn urlencoded_or_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    let urlencoded_filter = warp::body::form();
    let json_filter = warp::body::json();

    urlencoded_filter.or(json_filter).unify()
}

pub fn authorisation_optional(
    state: &ArcState,
) -> impl Filter<Extract = (Option<Actor>,), Error = Rejection> + Clone {
    let or_none_fn = |error: Rejection| async move {
        if error.find::<MissingHeader>().is_some() {
            Ok((None,))
        } else {
            Err(error)
        }
    };

    authorisation_required(state).map(Some).or_else(or_none_fn)
}

async fn authorise_user(
    state: ArcState,
    authorization_header: HeaderValue,
) -> Result<Actor, Rejection> {
    let credentials = Bearer::decode(&authorization_header).ok_or(Error::Unauthorized)?;
    let token = credentials.token();

    let access_token = OAuthToken::by_access_token(&state.db_pool, token)
        .await
        .map_err(Error::from)?;
    let actor = Actor::get(&state.db_pool, access_token.actor_id)
        .await
        .map_err(Error::from)?;

    Ok(actor)
}

pub fn authorisation_required(
    state: &ArcState,
) -> impl Filter<Extract = (Actor,), Error = Rejection> + Clone {
    crate::state::filter(state)
        .and(warp::header::value(AUTHORIZATION.as_ref()))
        .and_then(authorise_user)
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Enable CORS for all API endpoints
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb
    let cors = construct_cors(API_ALLOWED_METHODS);

    let v1_prefix = warp::path!("api" / "v1" / ..);

    let accounts = accounts::routes(state);
    let apps = apps::routes(state);
    let statuses = statuses::routes(state);
    let instance = instance::routes(state);

    let v1_routes = accounts.or(apps).or(statuses).or(instance);

    v1_prefix.and(v1_routes).with(cors)
}

pub mod accounts;
pub mod apps;
pub mod convert;
pub mod instance;
pub mod statuses;
