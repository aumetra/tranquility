use {
    crate::config::Configuration,
    sqlx::PgPool,
    std::{convert::Infallible, sync::Arc},
    warp::Filter,
};

#[allow(clippy::module_name_repetitions)]
/// State wrapped into an arc
pub type ArcState = Arc<State>;

/// Application-wide state
pub struct State {
    pub config: Configuration,
    pub db_pool: PgPool,
}

impl State {
    /// Create a new state instance wrapped into an Arc
    pub fn new(config: Configuration, db_pool: PgPool) -> ArcState {
        Arc::new(Self::new_arcless(config, db_pool))
    }

    /// Create a new state instance
    pub fn new_arcless(config: Configuration, db_pool: PgPool) -> Self {
        Self { config, db_pool }
    }
}

/// Create a filter that returns an arc-ed instance of the contained state
pub fn filter(state: &ArcState) -> impl Filter<Extract = (ArcState,), Error = Infallible> + Clone {
    let state = Arc::clone(state);

    warp::any().map(move || Arc::clone(&state))
}
