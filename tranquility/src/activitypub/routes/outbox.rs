use {
    super::{CollectionQuery, ACTIVITIES_PER_PAGE},
    crate::error::Error,
    itertools::Itertools,
    tranquility_types::activitypub::{
        collection::Item, Activity, Actor, Collection, OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
    },
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn outbox(user_id: Uuid, query: CollectionQuery) -> Result<impl Reply, Rejection> {
    #[allow(clippy::cast_possible_wrap)]
    let mut offset = query.offset.unwrap_or_default() as i64;

    // Set the offset to 0 in case someone decides to pass
    // a number that wraps the signed 64bit integer
    if offset < 0 {
        offset = 0;
    }

    let last_activities: Vec<Activity> =
        crate::database::object::select::by_type_and_owner("Create", &user_id, 10, offset)
            .await?
            .into_iter()
            .map(|db_object| serde_json::from_value(db_object.data).map_err(Error::from))
            .try_collect()?;
    let last_activities = last_activities.into_iter().map(Item::from).collect();

    let user_db = crate::database::actor::select::by_id(user_id).await?;
    let user: Actor = serde_json::from_value(user_db.actor).map_err(Error::from)?;

    let next = format!("{}?offset={}", user.outbox, offset + ACTIVITIES_PER_PAGE);

    let prev = if offset >= ACTIVITIES_PER_PAGE {
        offset - ACTIVITIES_PER_PAGE
    } else {
        0
    };
    let prev = format!("{}?offset={}", user.outbox, prev);

    let outbox_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.outbox.clone(),
        part_of: user.outbox,

        prev,
        next,

        ordered_items: last_activities,
        ..Collection::default()
    };

    Ok(warp::reply::json(&outbox_collection))
}
