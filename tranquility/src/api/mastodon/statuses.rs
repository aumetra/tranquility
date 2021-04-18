use {
    super::{
        authorisation_required, convert::IntoMastodon, restrict_body_size, urlencoded_or_json,
    },
    crate::{
        database::{Actor as DbActor, InsertExt, InsertObject},
        map_err,
        state::ArcState,
    },
    serde::Deserialize,
    std::sync::Arc,
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

async fn create(
    state: ArcState,
    author_db: DbActor,
    form: CreateForm,
) -> Result<Response, Rejection> {
    if state.config.instance.character_limit < form.status.chars().count() {
        return Ok(
            warp::reply::with_status("Status too long", StatusCode::BAD_REQUEST).into_response(),
        );
    }

    let author: Actor = map_err!(serde_json::from_value(author_db.actor))?;

    let (object_id, mut object) = crate::activitypub::instantiate::object(
        &state.config,
        author.id.as_str(),
        form.spoiler_text.as_str(),
        form.status.as_str(),
        form.sensitive,
        // TODO: Actually add collections to the to/cc array
        vec![PUBLIC_IDENTIFIER.into(), author.followers],
        vec![],
    );

    // Clean the summary and status from any malicious HTML
    crate::activitypub::clean_object(&mut object);

    let object_value = map_err!(serde_json::to_value(&object))?;

    InsertObject {
        id: object_id,
        owner_id: author_db.id,
        data: object_value,
    }
    .insert(&state.db_pool)
    .await?;

    let (_create_activity_id, create_activity) = crate::activitypub::instantiate::activity(
        &state.config,
        "Create",
        author.id.as_str(),
        object.clone(),
        object.to.clone(),
        object.cc.clone(),
    );

    crate::activitypub::deliverer::deliver(create_activity, Arc::clone(&state)).await?;

    let mastodon_status = object.into_mastodon(&state).await?;
    Ok(warp::reply::json(&mastodon_status).into_response())
}

pub fn routes(state: &ArcState) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let state_filter = crate::state::filter(state);

    warp::path!("statuses")
        .and(warp::post())
        .and(restrict_body_size(state))
        .and(state_filter)
        .and(authorisation_required(state))
        .and(urlencoded_or_json())
        .and_then(create)
}
