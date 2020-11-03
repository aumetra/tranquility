use {
    crate::error::Error,
    serde::Deserialize,
    tranquility_types::activitypub::{Activity, IsPrivate, Object},
    uuid::Uuid,
    warp::{Rejection, Reply},
};

#[derive(Deserialize)]
#[serde(untagged)]
enum ActivityObject {
    Activity(Activity),
    Object(Object),
}

impl IsPrivate for ActivityObject {
    fn is_private(&self) -> bool {
        match self {
            ActivityObject::Activity(activity) => activity.is_private(),
            ActivityObject::Object(object) => object.is_private(),
        }
    }
}

pub async fn objects(id: Uuid) -> Result<impl Reply, Rejection> {
    let object = crate::database::object::select::by_id(id).await?;
    let activity_or_object: ActivityObject =
        serde_json::from_value(object.data.clone()).map_err(Error::from)?;

    if activity_or_object.is_private() {
        return Err(warp::reject::not_found());
    }

    Ok(warp::reply::json(&object.data))
}
