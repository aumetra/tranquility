use {
    super::{authorization_required, convert::IntoMastodon},
    crate::database::model::Actor as DBActor,
    uuid::Uuid,
    warp::{Filter, Rejection, Reply},
};

async fn accounts(id: Uuid) -> Result<impl Reply, Rejection> {
    let db_actor = crate::database::actor::select::by_id(id).await?;
    let mastodon_account = db_actor.into_mastodon().await?;

    Ok(warp::reply::json(&mastodon_account))
}

async fn verify_credentials(db_actor: DBActor) -> Result<impl Reply, Rejection> {
    let mastodon_account = db_actor.into_mastodon().await?;

    Ok(warp::reply::json(&mastodon_account))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let accounts = warp::path!("api" / "v1" / "accounts" / Uuid)
        .and(warp::get())
        .and_then(accounts);

    let verify_credentials = warp::path!("api" / "v1" / "accounts" / "verify_credentials")
        .and(warp::get())
        .and(authorization_required())
        .and_then(verify_credentials);

    accounts.or(verify_credentials)
}
