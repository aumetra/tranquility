use {
    super::{authorisation_optional, authorisation_required, convert::IntoMastodon},
    crate::{
        activitypub::interactions, database::model::Actor as DbActor, error::Error, format_uuid,
        state::ArcState,
    },
    tranquility_types::{
        activitypub::Actor,
        mastodon::{Account, FollowResponse, Source},
    },
    uuid::Uuid,
    warp::{Filter, Rejection, Reply},
};

async fn accounts(
    id: Uuid,
    state: ArcState,
    authorized_db_actor: Option<DbActor>,
) -> Result<impl Reply, Rejection> {
    let db_actor = crate::database::actor::select::by_id(&state.db_pool, id).await?;
    let mut mastodon_account: Account = db_actor.into_mastodon(&state).await?;

    // Add the source field to the returned account if the requested account
    // is the account that has authorized itself
    if let Some(authorized_db_actor) = authorized_db_actor {
        if id == authorized_db_actor.id {
            let source: Source = authorized_db_actor.into_mastodon(&state).await?;
            mastodon_account.source = Some(source);
        }
    }

    Ok(warp::reply::json(&mastodon_account))
}

async fn follow(
    id: Uuid,
    state: ArcState,
    authorized_db_actor: DbActor,
) -> Result<impl Reply, Rejection> {
    let followed_db_actor = crate::database::actor::select::by_id(&state.db_pool, id).await?;
    let followed_actor: Actor =
        serde_json::from_value(followed_db_actor.actor).map_err(Error::from)?;

    interactions::follow(&state, authorized_db_actor, &followed_actor).await?;

    // TODO: Fill in information dynamically (followed by, blocked by, blocking, etc.)
    let follow_response = FollowResponse {
        id: format_uuid!(followed_db_actor.id),
        following: true,
        ..FollowResponse::default()
    };
    Ok(warp::reply::json(&follow_response))
}

async fn following(id: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let follow_activities =
        crate::database::object::select::by_type_and_owner(&state.db_pool, "Follow", &id, 10, 0)
            .await?;
    let followed_accounts: Vec<Account> = follow_activities.into_mastodon(&state).await?;

    Ok(warp::reply::json(&followed_accounts))
}

async fn followers(id: Uuid, state: ArcState) -> Result<impl Reply, Rejection> {
    let db_actor = crate::database::actor::select::by_id(&state.db_pool, id).await?;
    let actor: Actor = serde_json::from_value(db_actor.actor).map_err(Error::from)?;

    let followed_activities = crate::database::object::select::by_type_and_object_url(
        &state.db_pool,
        "Follow",
        actor.id.as_str(),
        10,
        0,
    )
    .await?;
    let follower_accounts: Vec<Account> = followed_activities.into_mastodon(&state).await?;

    Ok(warp::reply::json(&follower_accounts))
}

// TODO: Implement `/api/v1/accounts/:id/statuses` endpoint
/*async fn statuses(id: Uuid, authorized_db_actor: Option<DbActor>) -> Result<impl Reply, Rejection> {
}*/

async fn unfollow(
    id: Uuid,
    state: ArcState,
    authorized_db_actor: DbActor,
) -> Result<impl Reply, Rejection> {
    // Fetch the follow activity
    let followed_db_actor = crate::database::actor::select::by_id(&state.db_pool, id).await?;
    let followed_actor_id = format_uuid!(followed_db_actor.id);

    interactions::unfollow(&state, authorized_db_actor, followed_db_actor).await?;

    // TODO: Fill in information dynamically (followed by, blocked by, blocking, etc.)
    let unfollow_response = FollowResponse {
        id: followed_actor_id,
        ..FollowResponse::default()
    };
    Ok(warp::reply::json(&unfollow_response))
}

async fn verify_credentials(state: ArcState, db_actor: DbActor) -> Result<impl Reply, Rejection> {
    let mut mastodon_account: Account = db_actor.clone().into_mastodon(&state).await?;
    let mastodon_account_source: Source = db_actor.into_mastodon(&state).await?;

    mastodon_account.source = Some(mastodon_account_source);

    Ok(warp::reply::json(&mastodon_account))
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state_filter = crate::state::filter(state);

    let accounts = warp::path!("accounts" / Uuid)
        .and(warp::get())
        .and(state_filter.clone())
        .and(authorisation_optional(state))
        .and_then(accounts);

    let follow = warp::path!("accounts" / Uuid / "follow")
        .and(warp::post())
        .and(state_filter.clone())
        .and(authorisation_required(state))
        .and_then(follow);

    let following = warp::path!("accounts" / Uuid / "following")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(following);

    let followers = warp::path!("accounts" / Uuid / "followers")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(followers);

    /*let statuses = warp::path!("accounts" / Uuid / "statuses")
    .and(warp::get())
    .and(authorization_optional())
    .and_then(statuses);*/

    let unfollow = warp::path!("accounts" / Uuid / "unfollow")
        .and(warp::post())
        .and(state_filter.clone())
        .and(authorisation_required(state))
        .and_then(unfollow);

    let verify_credentials = warp::path!("accounts" / "verify_credentials")
        .and(warp::get())
        .and(state_filter)
        .and(authorisation_required(state))
        .and_then(verify_credentials);

    accounts
        .or(follow)
        .or(following)
        .or(followers)
        .or(unfollow)
        .or(verify_credentials)
}
