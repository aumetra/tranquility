use {
    super::{authorization_optional, authorization_required, convert::IntoMastodon},
    crate::database::model::Actor as DBActor,
    tranquility_types::mastodon::{Account, Source},
    uuid::Uuid,
    warp::{Filter, Rejection, Reply},
};

async fn accounts(id: Uuid, authorized_db_actor: Option<DBActor>) -> Result<impl Reply, Rejection> {
    let db_actor = crate::database::actor::select::by_id(id).await?;
    let mut mastodon_account: Account = db_actor.into_mastodon().await?;

    // Add the source field to the returned account if the requested account
    // is the account that has authorized itself
    if let Some(authorized_db_actor) = authorized_db_actor {
        if id == authorized_db_actor.id {
            let source: Source = authorized_db_actor.into_mastodon().await?;
            mastodon_account.source = Some(source);
        }
    }

    Ok(warp::reply::json(&mastodon_account))
}

async fn verify_credentials(db_actor: DBActor) -> Result<impl Reply, Rejection> {
    let mut mastodon_account: Account = db_actor.into_mastodon_cloned().await?;
    let mastodon_account_source: Source = db_actor.into_mastodon().await?;

    mastodon_account.source = Some(mastodon_account_source);

    Ok(warp::reply::json(&mastodon_account))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let accounts = warp::path!("api" / "v1" / "accounts" / Uuid)
        .and(warp::get())
        .and(authorization_optional())
        .and_then(accounts);

    let verify_credentials = warp::path!("api" / "v1" / "accounts" / "verify_credentials")
        .and(warp::get())
        .and(authorization_required())
        .and_then(verify_credentials);

    accounts.or(verify_credentials)
}
