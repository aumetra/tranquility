use {
    crate::{database::model::Actor as DBActor, error::Error},
    serde::Deserialize,
    tranquility_types::{
        activitypub::Actor,
        webfinger::{Link, Resource},
    },
    warp::{Filter, Rejection, Reply},
};

// Keeping this for future use
#[allow(dead_code)]
pub async fn fetch_actor(username: &str, domain: &str) -> Result<(Actor, DBActor), Error> {
    let resource = format!("acct:{}@{}", username, domain);
    let url = format!(
        "https://{}/.well-known/webfinger?resource={}",
        domain, resource
    );

    let client = &crate::REQWEST_CLIENT;
    let request = client
        .get(&url)
        .header("Accept", "application/jrd+json")
        .build()?;
    let resource: Resource = client.execute(request).await?.json().await?;

    let actor_url = resource
        .links
        .iter()
        .find(|link| link.rel == "self")
        .ok_or(Error::UnexpectedWebfingerResource)?;

    crate::activitypub::fetcher::fetch_actor(&actor_url.href).await
}

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
