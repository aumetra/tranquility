use {
    super::{CollectionQuery, ACTIVITIES_PER_PAGE},
    crate::{activitypub::FollowActivity, error::Error},
    tranquility_types::activitypub::{
        collection::Item, Actor, Collection, OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE,
    },
    uuid::Uuid,
    warp::{Rejection, Reply},
};

pub async fn following(user_id: Uuid, query: CollectionQuery) -> Result<impl Reply, Rejection> {
    #[allow(clippy::cast_possible_wrap)]
    let mut offset = query.offset.unwrap_or_default() as i64;

    // Set the offset to 0 in case someone decides to pass
    // a number that wraps the signed 64bit integer
    if offset < 0 {
        offset = 0;
    }

    let last_follow_activities = crate::database::object::select::by_type_and_owner(
        "Follow",
        &user_id,
        ACTIVITIES_PER_PAGE,
        offset,
    )
    .await?
    .into_iter()
    .map(|object| serde_json::from_value(object.data).map_err(Error::from))
    .collect::<Result<Vec<FollowActivity>, Error>>()?;

    let last_followed = last_follow_activities
        .into_iter()
        .map(|activity| activity.activity.object.as_url().unwrap().to_owned())
        .map(Item::from)
        .collect::<Vec<Item>>();

    let user_db = crate::database::actor::select::by_id(user_id).await?;
    let user: Actor = serde_json::from_value(user_db.actor).map_err(Error::from)?;

    let next = format!("{}?offset={}", user.following, offset + ACTIVITIES_PER_PAGE);

    let prev = if offset >= ACTIVITIES_PER_PAGE {
        offset - ACTIVITIES_PER_PAGE
    } else {
        0
    };
    let prev = format!("{}?offset={}", user.following, prev);

    let following_collection = Collection {
        r#type: OUTBOX_FOLLOW_COLLECTIONS_PAGE_TYPE.into(),

        id: user.following.clone(),
        part_of: user.following,

        prev,
        next,

        ordered_items: last_followed,
        ..Collection::default()
    };

    Ok(warp::reply::json(&following_collection))
}
