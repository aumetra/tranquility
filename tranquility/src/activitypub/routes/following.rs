use super::CollectionQuery;
use crate::error::Error;
use crate::{
    activitypub::FollowActivity, consts::activitypub::ACTIVITIES_PER_PAGE,
    database::Actor as DbActor, format_uuid, state::ArcState,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};
use itertools::Itertools;
use tranquility_types::activitypub::{
    collection::Item, Actor, Collection, OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
};
use uuid::Uuid;

pub async fn following(
    Path(user_id): Path<Uuid>,
    Extension(state): Extension<ArcState>,
    Query(query): Query<CollectionQuery>,
) -> Result<impl IntoResponse, Error> {
    let latest_follow_activities = crate::database::follow::following(
        &state.db_pool,
        user_id,
        query.last_id,
        ACTIVITIES_PER_PAGE,
    )
    .await?;
    let last_id = latest_follow_activities
        .last()
        .map(|activity| format_uuid!(activity.id))
        .unwrap_or_default();

    let latest_followed = latest_follow_activities
        .into_iter()
        .filter_map(|activity| {
            let follow_activity: FollowActivity = serde_json::from_value(activity.data).ok()?;
            let followed_url = follow_activity.activity.object.as_url()?.clone();

            Some(Item::Url(followed_url))
        })
        .collect_vec();

    let user_db = DbActor::get(&state.db_pool, user_id).await?;
    let user: Actor = serde_json::from_value(user_db.actor)?;

    let next = format!("{}?last_id={}", user.following, last_id);

    let following_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.following.clone(),
        part_of: user.following,

        next,

        ordered_items: latest_followed,
        ..Collection::default()
    };

    Ok(Json(following_collection))
}
