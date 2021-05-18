use {
    crate::{consts::daemon::DELETE_INTERVAL, database::OAuthAuthorization, state::ArcState},
    std::future::Future,
    tokio::time,
};

// Keeping this for future use
#[allow(dead_code)]
fn bulk_spawn(futures: Vec<impl Future<Output = ()> + Send + Sync + 'static>) {
    for future in futures {
        tokio::spawn(future);
    }
}

/// Delete all expired authorisation codes from the database
async fn delete_expired_authorisation_codes(state: ArcState) {
    let mut query_interval = time::interval(DELETE_INTERVAL);

    loop {
        match OAuthAuthorization::delete_expired(&state.db_pool).await {
            Ok(_) => (),
            Err(err) => warn!(error = ?err, "Couldn't delete expired tokens"),
        }

        query_interval.tick().await;
    }
}

pub fn start(state: ArcState) {
    tokio::spawn(delete_expired_authorisation_codes(state));
}
