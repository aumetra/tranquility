use {
    crate::error::Error,
    serde::Deserialize,
    tranquility_types::{
        activitypub::Actor,
        webfinger::{Link, Resource},
    },
    warp::{Filter, Rejection, Reply},
};

#[derive(Deserialize)]
pub struct Query {
    resource: String,
}

pub async fn webfinger(query: Query) -> Result<impl Reply, Rejection> {
    let resource = query.resource;
    let username = resource
        .trim_start_matches("acct:")
        .split('@')
        .next()
        .ok_or(Error::InvalidRequest)?;

    let actor_db = crate::database::actor::select::by_username_local(username).await?;
    let actor: Actor = serde_json::from_value(actor_db.actor).map_err(Error::from)?;

    let link = Link {
        rel: "self".into(),
        r#type: Some("application/activity+json".into()),
        href: actor.id.clone(),
        ..Link::default()
    };
    let resource = Resource {
        subject: resource,

        aliases: vec![actor.id],

        links: vec![link],
        ..Resource::default()
    };

    Ok(warp::reply::with_header(
        warp::reply::json(&resource),
        "Content-Type",
        "application/jrd+json",
    ))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path!(".well-known" / "webfinger")
        .and(warp::query())
        .and_then(webfinger)
}
