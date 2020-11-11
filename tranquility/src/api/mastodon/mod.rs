use {
    crate::{database::model::Actor, error::Error},
    serde::de::DeserializeOwned,
    warp::{Filter, Rejection, Reply},
};

pub fn form_urlencoded_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::form().or(warp::body::json()).unify()
}

pub fn authorization_required() -> impl Filter<Extract = (Actor,), Error = Rejection> + Copy {
    warp::header("authorization").and_then(|authorization_header: String| async move {
        let token = {
            let mut split_header = authorization_header.split_whitespace();
            if split_header
                .next()
                .ok_or(Error::Unauthorized)?
                .to_lowercase()
                != "Bearer"
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
    let accounts = accounts::routes();

    let apps = warp::path!("api" / "v1" / "apps")
        .and(warp::post())
        .and(form_urlencoded_json())
        .and_then(apps::create);

    accounts.or(apps)
}

pub mod accounts;
pub mod apps;
pub mod convert;
