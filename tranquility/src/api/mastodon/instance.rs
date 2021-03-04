use {
    crate::{config::ArcConfig, consts::VERSION},
    tranquility_types::mastodon::{
        instance::{Stats, Urls},
        Instance,
    },
    warp::{Filter, Rejection, Reply},
};

async fn instance(config: ArcConfig) -> Result<impl Reply, Rejection> {
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

pub fn routes(
    config: ArcConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let config = crate::config::filter(config);

    warp::path!("instance").and(config).and_then(instance)
}
