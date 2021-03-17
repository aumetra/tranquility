use {
    crate::config::Configuration,
    sqlx::PgPool,
    std::{convert::Infallible, sync::Arc},
    warp::Filter,
};

#[allow(clippy::module_name_repetitions)]
pub type ArcState = Arc<State>;

pub struct State {
    pub config: Configuration,
    pub db_pool: PgPool,
}

impl State {
    pub fn new(config: Configuration, db_pool: PgPool) -> ArcState {
        Arc::new(Self::new_without_arc(config, db_pool))
    }

    pub fn new_without_arc(config: Configuration, db_pool: PgPool) -> Self {
        Self { config, db_pool }
    }
}

pub fn filter(state: &ArcState) -> impl Filter<Extract = (ArcState,), Error = Infallible> + Clone {
    let state = Arc::clone(state);

    warp::any().map(move || Arc::clone(&state))
}
