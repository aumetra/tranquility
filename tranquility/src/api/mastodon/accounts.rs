use super::{convert::IntoMastodon, Auth};
use crate::{
    activitypub::interactions,
    database::{Actor as DbActor, Object as DbObject},
    error::Error,
    format_uuid,
    state::ArcState,
};
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use tranquility_types::{
    activitypub::Actor,
    mastodon::{Account, FollowResponse, Source},
};
use uuid::Uuid;

async fn accounts(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
    authorized_db_actor: Option<Auth>,
) -> Result<impl IntoResponse, Error> {
    let db_actor = DbActor::get(&state.db_pool, id).await?;
    let mut mastodon_account: Account = db_actor.into_mastodon(&state).await?;

    // Add the source field to the returned account if the requested account
    // is the account that has authorized itself
    if let Some(Auth(authorized_db_actor)) = authorized_db_actor {
        if id == authorized_db_actor.id {
            let source: Source = authorized_db_actor.into_mastodon(&state).await?;
            mastodon_account.source = Some(source);
        }
    }

    Ok(Json(&mastodon_account))
}

async fn follow(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
    Auth(authorized_db_actor): Auth,
) -> Result<impl IntoResponse, Error> {
    let followed_db_actor = DbActor::get(&state.db_pool, id).await?;
    let followed_actor: Actor = serde_json::from_value(followed_db_actor.actor)?;

    interactions::follow(&state, authorized_db_actor, &followed_actor).await?;

    // TODO: Fill in information dynamically (followed by, blocked by, blocking, etc.)
    let follow_response = FollowResponse {
        id: format_uuid!(followed_db_actor.id),
        following: true,
        ..FollowResponse::default()
    };
    Ok(Json(&follow_response))
}

async fn following(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
) -> Result<impl IntoResponse, Error> {
    let follow_activities =
        DbObject::by_type_and_owner(&state.db_pool, "Follow", &id, 10, 0).await?;
    let followed_accounts: Vec<Account> = follow_activities.into_mastodon(&state).await?;

    Ok(Json(&followed_accounts))
}

async fn followers(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
) -> Result<impl IntoResponse, Error> {
    let db_actor = DbActor::get(&state.db_pool, id).await?;
    let actor: Actor = serde_json::from_value(db_actor.actor)?;

    let followed_activities =
        DbObject::by_type_and_object_url(&state.db_pool, "Follow", actor.id.as_str(), 10, 0)
            .await?;
    let follower_accounts: Vec<Account> = followed_activities.into_mastodon(&state).await?;

    Ok(Json(&follower_accounts))
}

// TODO: Implement `/api/v1/accounts/:id/statuses` endpoint
/*async fn statuses(Path(id): Path<Uuid>, authorized_db_actor: Option<Auth>) -> Result<impl Reply, Rejection> {
}*/

async fn unfollow(
    Path(id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
    Auth(authorized_db_actor): Auth,
) -> Result<impl IntoResponse, Error> {
    // Fetch the follow activity
    let followed_db_actor = DbActor::get(&state.db_pool, id).await?;
    let followed_actor_id = format_uuid!(followed_db_actor.id);

    interactions::unfollow(&state, authorized_db_actor, followed_db_actor).await?;

    // TODO: Fill in information dynamically (followed by, blocked by, blocking, etc.)
    let unfollow_response = FollowResponse {
        id: followed_actor_id,
        ..FollowResponse::default()
    };
    Ok(Json(&unfollow_response))
}

async fn verify_credentials(
    Extension(state): Extension<ArcState>,
    Auth(db_actor): Auth,
) -> Result<impl IntoResponse, Error> {
    let mut mastodon_account: Account = db_actor.clone().into_mastodon(&state).await?;
    let mastodon_account_source: Source = db_actor.into_mastodon(&state).await?;

    mastodon_account.source = Some(mastodon_account_source);

    Ok(Json(&mastodon_account))
}

pub fn routes() -> Router {
    Router::new()
        .route("/accounts/:id", get(accounts))
        .route("/accounts/:id/follow", post(follow))
        .route("/accounts/:id/following", get(following))
        .route("/accounts/:id/followers", get(followers))
        //.route("/accounts/:id/statuses", get(statuses))
        .route("/accounts/:id/unfollow", post(unfollow))
        .route("/accounts/verify_credentials", get(verify_credentials))
}
