use {
    crate::{database::model::Actor, error::Error},
    once_cell::sync::Lazy,
    serde::de::DeserializeOwned,
    tranquility_types::mastodon::App,
    warp::{reject::MissingHeader, Filter, Rejection, Reply},
};

static DEFAULT_APPLICATION: Lazy<App> = Lazy::new(|| App {
    name: "Web".into(),
    ..App::default()
});

pub fn urlencoded_or_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::form().or(warp::body::json()).unify()
}

pub fn authorization_optional() -> impl Filter<Extract = (Option<Actor>,), Error = Rejection> + Copy
{
    authorization_required()
        .map(Some)
        .or_else(|error: Rejection| async move {
            if error.find::<MissingHeader>().is_some() {
                Ok((None,))
            } else {
                Err(error)
            }
        })
}

pub fn authorization_required() -> impl Filter<Extract = (Actor,), Error = Rejection> + Copy {
    warp::header("authorization").and_then(|authorization_header: String| async move {
        let token = {
            let mut split_header = authorization_header.split_whitespace();
            if split_header
                .next()
                .ok_or(Error::Unauthorized)?
                .to_lowercase()
                != "bearer"
            {
                return Err::<_, Rejection>(Error::Unauthorized.into());
            }

            split_header.next().ok_or(Error::Unauthorized)?
        };

        let access_token = crate::database::oauth::token::select::by_token(token).await?;
        let actor = crate::database::actor::select::by_id(access_token.actor_id).await?;

        Ok(actor)
    })
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let v1_prefix = warp::path!("api" / "v1" / ..);

    let accounts = accounts::routes();
    let apps = apps::routes();
    let statuses = statuses::routes();

    let v1_routes = accounts.or(apps).or(statuses);

    v1_prefix.and(v1_routes)
}

pub mod accounts;
pub mod apps;
pub mod convert;
pub mod statuses;
