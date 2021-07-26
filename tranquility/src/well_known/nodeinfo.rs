use {
    crate::consts::{SOFTWARE_NAME, VERSION},
    tranquility_types::nodeinfo::{Link, LinkCollection, Nodeinfo, Services, Software, Usage},
    warp::{Filter, Rejection, Reply},
};

async fn nodeinfo() -> Result<impl Reply, Rejection> {
    let state = crate::state::get();

    let info = Nodeinfo {
        protocols: vec!["activitypub".into()],
        software: Software {
            name: SOFTWARE_NAME.into(),
            version: VERSION.into(),
            ..Software::default()
        },
        services: Services {
            inbound: Vec::new(),
            outbound: Vec::new(),
        },
        open_registrations: !state.config.instance.closed_registrations,
        usage: Usage::default(),
        ..Nodeinfo::default()
    };

    Ok(warp::reply::json(&info))
}

async fn nodeinfo_links() -> Result<impl Reply, Rejection> {
    let state = crate::state::get();

    let entity_link = format!(
        "https://{}/.well-known/nodeinfo/2.1",
        state.config.instance.domain
    );

    let link = Link::new(entity_link);
    let link_collection = LinkCollection { links: vec![link] };

    Ok(warp::reply::json(&link_collection))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let nodeinfo_links = warp::path!("nodeinfo").and_then(nodeinfo_links);
    let nodeinfo_entity = warp::path!("nodeinfo" / "2.1").and_then(nodeinfo);

    nodeinfo_links.or(nodeinfo_entity)
}
