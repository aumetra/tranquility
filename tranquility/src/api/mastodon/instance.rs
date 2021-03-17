use {
    crate::{consts::VERSION, state::ArcState},
    tranquility_types::mastodon::{
        instance::{Stats, Urls},
        Instance,
    },
    warp::{Filter, Rejection, Reply},
};

async fn instance(state: ArcState) -> Result<impl Reply, Rejection> {
    let streaming_api = format!("wss://{}", state.config.instance.domain);

    let instance = Instance {
        version: VERSION.into(),
        title: state.config.instance.domain.clone(),
        uri: state.config.instance.domain.clone(),
        short_description: None,
        description: state.config.instance.description.clone(),

        urls: Urls { streaming_api },
        stats: Stats { ..Stats::default() },

        registrations: !state.config.instance.closed_registrations,
        invites_enabled: false,
        approval_required: false,

        email: None,
        contact_account: None,

        ..Instance::default()
    };

    Ok(warp::reply::json(&instance))
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state = crate::state::filter(state);

    warp::path!("instance").and(state).and_then(instance)
}
