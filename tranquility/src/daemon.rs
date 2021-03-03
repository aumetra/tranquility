use {crate::consts::daemon::DELETE_INTERVAL, std::future::Future, tokio::time};

// Keeping this for future use
#[allow(dead_code)]
fn bulk_spawn(futures: Vec<impl Future<Output = ()> + Send + Sync + 'static>) {
    for future in futures {
        tokio::spawn(future);
    }
}

async fn delete_expired_authorization_codes() {
    let mut query_interval = time::interval(DELETE_INTERVAL);

    loop {
        match crate::database::oauth::authorization::delete::expired().await {
            Ok(_) => (),
            Err(err) => warn!(error = ?err, "Couldn't delete expired tokens"),
        }

        query_interval.tick().await;
    }
}

pub fn start() {
    tokio::spawn(delete_expired_authorization_codes());
}
