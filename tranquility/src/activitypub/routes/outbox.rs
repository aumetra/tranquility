use {
    super::CollectionQuery,
    crate::{
        consts::activitypub::ACTIVITIES_PER_PAGE, database::Actor as DbActor, format_uuid,
        state::ArcState,
    },
    itertools::Itertools,
    std::ops::Not,
    tranquility_types::activitypub::{
        collection::Item, Activity, Actor, Collection, IsPrivate,
        OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
    },
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn outbox(
    user_id: Uuid,
    state: ArcState,
    query: CollectionQuery,
) -> Result<impl Reply, Rejection> {
    let latest_activities = crate::database::outbox::activities(
        &state.db_pool,
        user_id,
        query.last_id,
        ACTIVITIES_PER_PAGE,
    )
    .await?;
    let last_id = latest_activities
        .last()
        .map(|activity| format_uuid!(activity.id))
        .unwrap_or_default();

    let latest_activities = latest_activities
        .into_iter()
        .filter_map(|activity| {
            let create_activity: Activity = serde_json::from_value(activity.data).ok()?;

            create_activity
                .is_private()
                .not()
                .then(|| Item::Activity(create_activity))
        })
        .collect_vec();

    let user_db = DbActor::get(&state.db_pool, user_id).await?;
    let user: Actor = serde_json::from_value(user_db.actor)?;

    let next = format!("{}?last_id={}", user.outbox, last_id);

    let outbox_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.outbox.clone(),
        part_of: user.outbox,

        next,

        ordered_items: latest_activities,
        ..Collection::default()
    };

    Ok(warp::reply::json(&outbox_collection))
}
