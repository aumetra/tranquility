use {
    crate::{database::model::Actor as DbActor, error::Error},
    serde::Deserialize,
    tranquility_types::{
        activitypub::Actor,
        webfinger::{Link, Resource},
    },
    warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply},
};

// Keeping this for future use
#[allow(dead_code)]
pub async fn fetch_actor(username: &str, domain: &str) -> Result<(Actor, DbActor), Error> {
    let resource = format!("acct:{}@{}", username, domain);
    let url = format!(
        "https://{}/.well-known/webfinger?resource={}",
        domain, resource
    );

    let client = &crate::util::REQWEST_CLIENT;
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

pub async fn webfinger(query: Query) -> Result<Response, Rejection> {
    let resource = query.resource;
    let mut resource_tokens = resource.trim_start_matches("acct:").split('@');

    let username = resource_tokens.next().ok_or(Error::InvalidRequest)?;

    let config = crate::config::get();
    if resource_tokens.next().ok_or(Error::InvalidRequest)? != config.instance.domain {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

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
    )
    .into_response())
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Enable CORS for the ".well-known" routes
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb
    let cors = warp::cors().allow_any_origin().build();

    warp::path!(".well-known" / "webfinger")
        .and(warp::query())
        .and_then(webfinger)
        .with(cors)
}
