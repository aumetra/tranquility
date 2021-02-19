use {
    super::CollectionQuery,
    crate::{activitypub::FollowActivity, consts::activitypub::ACTIVITIES_PER_PAGE, error::Error},
    itertools::Itertools,
    tranquility_types::activitypub::{
        collection::Item, Actor, Collection, OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
    },
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn followers(user_id: Uuid, query: CollectionQuery) -> Result<impl Reply, Rejection> {
    #[allow(clippy::cast_possible_wrap)]
    let mut offset = query.offset.unwrap_or_default() as i64;

    // Set the offset to 0 in case someone decides to pass
    // a number that wraps the signed 64bit integer
    if offset < 0 {
        offset = 0;
    }

    let actor_db = crate::database::actor::select::by_id(user_id).await?;
    let actor: Actor = serde_json::from_value(actor_db.actor).map_err(Error::from)?;
    let last_follow_activities: Vec<FollowActivity> =
        crate::database::object::select::by_type_and_object_url(
            "Follow",
            &actor.id,
            ACTIVITIES_PER_PAGE,
            offset,
        )
        .await?
        .into_iter()
        .map(|object| serde_json::from_value(object.data).map_err(Error::from))
        .try_collect()?;

    let last_followed = last_follow_activities
        .into_iter()
        .map(|activity| activity.activity.actor)
        .map(Item::from)
        .collect_vec();

    let user_db = crate::database::actor::select::by_id(user_id).await?;
    let user: Actor = serde_json::from_value(user_db.actor).map_err(Error::from)?;

    let next = format!("{}?offset={}", user.followers, offset + ACTIVITIES_PER_PAGE);

    let prev = if offset >= ACTIVITIES_PER_PAGE {
        offset - ACTIVITIES_PER_PAGE
    } else {
        0
    };
    let prev = format!("{}?offset={}", user.followers, prev);

    let followers_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.followers.clone(),
        part_of: user.followers,

        prev,
        next,

        ordered_items: last_followed,
        ..Collection::default()
    };

    Ok(warp::reply::json(&followers_collection))
}
