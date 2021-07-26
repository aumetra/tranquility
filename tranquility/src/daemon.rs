use crate::{consts::daemon::DELETE_INTERVAL, database::OAuthAuthorization};

/// Delete all expired authorisation codes from the database
async fn delete_expired_authorisation_codes() {
    let state = crate::state::get_full();
    let mut query_interval = tokio::time::interval(DELETE_INTERVAL);

    loop {
        match OAuthAuthorization::delete_expired(&state.db_pool).await {
            Ok(_) => (),
            Err(err) => warn!(error = ?err, "Couldn't delete expired tokens"),
        }

        query_interval.tick().await;
    }
}

pub fn start() {
    tokio::spawn(delete_expired_authorisation_codes());
}
