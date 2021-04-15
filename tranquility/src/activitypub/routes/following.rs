use {
    super::CollectionQuery,
    crate::{
        activitypub::FollowActivity, consts::activitypub::ACTIVITIES_PER_PAGE,
        database::Actor as DbActor, format_uuid, map_err, state::ArcState,
    },
    itertools::Itertools,
    ormx::Table,
    tranquility_types::activitypub::{
        collection::Item, Actor, Collection, OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
    },
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn following(
    user_id: Uuid,
    state: ArcState,
    query: CollectionQuery,
) -> Result<impl Reply, Rejection> {
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
            let followed_url = follow_activity.activity.object.as_url()?.to_owned();

            Some(Item::Url(followed_url))
        })
        .collect_vec();

    let user_db = map_err!(DbActor::get(&state.db_pool, user_id).await)?;
    let user: Actor = map_err!(serde_json::from_value(user_db.actor))?;

    let next = format!("{}?last_id={}", user.following, last_id);

    let following_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.following.clone(),
        part_of: user.following,

        next,

        ordered_items: latest_followed,
        ..Collection::default()
    };

    Ok(warp::reply::json(&following_collection))
}
