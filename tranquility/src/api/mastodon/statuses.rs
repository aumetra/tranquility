use super::{convert::IntoMastodon, Authorisation};
use crate::{
    activitypub::Clean,
    consts::MAX_BODY_SIZE,
    database::{InsertExt, InsertObject},
    error::Error,
    state::ArcState,
    util::{mention::FormatMention, Form},
};
use axum::{
    extract::ContentLengthLimit, http::StatusCode, response::IntoResponse, routing::post,
    Extension, Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tranquility_types::activitypub::{Actor, PUBLIC_IDENTIFIER};

#[cfg(feature = "markdown")]
use crate::api::ParseMarkdown;

#[derive(Deserialize)]
struct CreateForm {
    status: String,

    #[serde(default)]
    sensitive: bool,
    #[serde(default)]
    spoiler_text: String,
}

async fn create(
    Extension(state): Extension<ArcState>,
    Authorisation(author_db): Authorisation,
    ContentLengthLimit(Form(form)): ContentLengthLimit<Form<CreateForm>, MAX_BODY_SIZE>,
) -> Result<impl IntoResponse, Error> {
    if state.config.instance.character_limit < form.status.chars().count() {
        return Ok((StatusCode::BAD_REQUEST, "Status too long").into_response());
    }

    let author: Actor = serde_json::from_value(author_db.actor)?;

    let (object_id, mut object) = crate::activitypub::instantiate::object(
        &state.config,
        "Note",
        author.id.as_str(),
        form.spoiler_text.as_str(),
        form.status.as_str(),
        form.sensitive,
        // TODO: Actually add collections to the to/cc array
        vec![PUBLIC_IDENTIFIER.into(), author.followers],
        vec![],
    );

    object.format_mentions(Arc::clone(&state)).await;

    // Parse the markdown if the feature is enabled
    #[cfg(feature = "markdown")]
    object.parse_markdown();

    object.clean();

    let object_value = serde_json::to_value(&object)?;

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
    Ok(Json(&mastodon_status).into_response())
}

pub fn routes() -> Router {
    Router::new().route("/statuses", post(create))
}
