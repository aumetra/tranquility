use notify::RecursiveMode;

use {
    crate::config::Configuration,
    arc_swap::{ArcSwap, Guard},
    notify::{Event, EventKind, Watcher},
    once_cell::sync::OnceCell,
    sqlx::PgPool,
    std::{path::Path, sync::Arc},
    tokio::runtime::Handle,
};

static STATE: OnceCell<ArcSwap<State>> = OnceCell::new();

/// Application-wide state
pub struct State {
    pub config: Configuration,
    pub db_pool: PgPool,
}

impl State {
    /// Create a new state instance
    pub fn new(config: Configuration, db_pool: PgPool) -> Arc<Self> {
        Arc::new(Self::new_arcless(config, db_pool))
    }

    /// Create a new state instance without an arc
    pub fn new_arcless(config: Configuration, db_pool: PgPool) -> Self {
        Self { config, db_pool }
    }
}

/// Load the configuration and connect to the database
#[inline]
async fn prepare_state<P>(path: P) -> Arc<State>
where
    P: AsRef<Path>,
{
    let config = crate::config::load(path).await;
    let db_pool = crate::database::connection::init_pool(&config.server.database_url)
        .await
        .expect("Couldn't connect to database");

    // It's maybe a bit excessive the migrate the database everytime the configuration file is changed
    // But the database URL might change, so..
    crate::database::migrate(&db_pool)
        .await
        .expect("Failed to migrate the database");

    State::new(config, db_pool)
}

/// Initialise the configuration OnceCell
pub async fn init<P>(path: P)
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_path_buf();

    let state = prepare_state(&path).await;
    init_raw(state);

    // Obtain a handle to the current runtime for use inside the event function
    let handle = Handle::current();

    let mut watcher = {
        let path = path.clone();

        notify::recommended_watcher(move |event| {
            // Other events don't really make sense
            match event {
                Ok(Event {
                    kind: EventKind::Modify(..),
                    ..
                }) => {
                    let state = handle.block_on(prepare_state(&path));
                    STATE.get().unwrap().swap(state);

                    #[cfg(feature = "email")]
                    crate::email::update_transport();
                }
                Err(err) => warn!(error = ?err, "File watching failed"),
                _ => (),
            }
        })
        .expect("Failed to initialise file watcher")
    };

    watcher
        .watch(&path, RecursiveMode::NonRecursive)
        .expect("Failed to watch configuration file");
}

/// Initialise the state from a raw struct
pub fn init_raw(state: Arc<State>) {
    let state = ArcSwap::new(state);

    STATE
        .set(state)
        .map_err(|_| ())
        .expect("State OnceCell already initialised");
}

/// Get a reference to the global configuration
#[inline]
pub fn get() -> Guard<Arc<State>> {
    STATE.get().expect("State uninitialised").load()
}

/// Get a clone of the inner arc
///
/// Doesn't take up a cheap proxy. Useful for long running tasks
#[inline]
pub fn get_full() -> Arc<State> {
    STATE.get().expect("State uninitialised").load_full()
}
