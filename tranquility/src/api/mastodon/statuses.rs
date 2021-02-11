use {
    super::{authorization_required, urlencoded_or_json},
    crate::{database::model::Actor as DbActor, error::Error},
    serde::Deserialize,
    tranquility_types::activitypub::{Actor, PUBLIC_IDENTIFIER},
    warp::{http::StatusCode, reply::Response, Filter, Rejection, Reply},
};

#[derive(Deserialize)]
struct CreateForm {
    status: String,

    #[serde(default)]
    sensitive: bool,
    #[serde(default)]
    spoiler_text: String,
}

async fn create(author_db: DbActor, form: CreateForm) -> Result<Response, Rejection> {
    let config = crate::config::get();
    if config.instance.character_limit < form.status.chars().count() {
        return Ok(
            warp::reply::with_status("Status too long", StatusCode::BAD_REQUEST).into_response(),
        );
    }

    let author: Actor = serde_json::from_value(author_db.actor).map_err(Error::from)?;

    let (object_id, mut object) = crate::activitypub::instantiate::object(
        author.id.as_str(),
        form.spoiler_text.as_str(),
        form.status.as_str(),
        form.sensitive,
        // TODO: Actually add collections to the to/cc array
        vec![PUBLIC_IDENTIFIER.into()],
        vec![],
    );

    // Clean the summary and status from any malicious HTML
    crate::activitypub::clean_object(&mut object);

    let object_value = serde_json::to_value(&object).map_err(Error::from)?;
    crate::database::object::insert(object_id, author_db.id, object_value).await?;

    let (_create_activity_id, create_activity) = crate::activitypub::instantiate::activity(
        "Create",
        author.id.as_str(),
        object.clone(),
        object.to,
        object.cc,
    );

    crate::activitypub::deliverer::deliver(create_activity).await?;

    Ok("".into_response())
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path!("statuses")
        .and(authorization_required())
        .and(urlencoded_or_json())
        .and_then(create)
}
