use {
    crate::{
        consts::cors::GENERAL_ALLOWED_METHODS, database::Actor as DbActor, error::Error, map_err,
        state::ArcState, util::construct_cors,
    },
    serde::Deserialize,
    tranquility_types::{
        activitypub::Actor,
        webfinger::{Link, Resource},
    },
    warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply},
};

// Keeping this for future use
#[allow(dead_code)]
pub async fn fetch_actor(
    state: &ArcState,
    username: &str,
    domain: &str,
) -> Result<(Actor, DbActor), Error> {
    let resource = format!("acct:{}@{}", username, domain);
    let url = format!(
        "https://{}/.well-known/webfinger?resource={}",
        domain, resource
    );

    let client = &crate::util::HTTP_CLIENT;
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

    crate::activitypub::fetcher::fetch_actor(state, &actor_url.href).await
}

#[derive(Deserialize)]
pub struct Query {
    resource: String,
}

pub async fn webfinger(state: ArcState, query: Query) -> Result<Response, Rejection> {
    let resource = query.resource;
    let mut resource_tokens = resource.trim_start_matches("acct:").split('@');

    let username = resource_tokens.next().ok_or(Error::InvalidRequest)?;

    if resource_tokens.next().ok_or(Error::InvalidRequest)? != state.config.instance.domain {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let actor_db = DbActor::by_username_local(&state.db_pool, username).await?;
    let actor: Actor = map_err!(serde_json::from_value(actor_db.actor))?;

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

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state = crate::state::filter(state);

    // Enable CORS for the ".well-known" routes
    // See: https://github.com/tootsuite/mastodon/blob/85324837ea1089c00fb4aefc31a7242847593b52/config/initializers/cors.rb
    let cors = construct_cors(GENERAL_ALLOWED_METHODS);

    warp::path!(".well-known" / "webfinger")
        .and(state)
        .and(warp::query())
        .and_then(webfinger)
        .with(cors)
}
