use {
    crate::consts::VERSION,
    tranquility_types::mastodon::{
        instance::{Stats, Urls},
        Instance,
    },
    warp::{Filter, Rejection, Reply},
};

pub async fn instance() -> Result<impl Reply, Rejection> {
    let config = crate::config::get();

    let streaming_api = format!("wss://{}", config.instance.domain);

    let instance = Instance {
        version: VERSION.into(),
        title: config.instance.domain.clone(),
        uri: config.instance.domain.clone(),
        short_description: None,
        description: config.instance.description.clone(),

        urls: Urls { streaming_api },
        stats: Stats { ..Stats::default() },

        registrations: !config.instance.closed_registrations,
        invites_enabled: false,
        approval_required: false,

        email: None,
        contact_account: None,

        ..Instance::default()
    };

    Ok(warp::reply::json(&instance))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path!("instance").and_then(instance)
}
