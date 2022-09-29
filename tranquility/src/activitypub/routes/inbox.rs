use crate::{activitypub::fetcher, crypto, error::Error, match_handler, state::ArcState};
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use std::{error::Error as StdError, sync::Arc};
use tranquility_types::activitypub::{activity::ObjectField, Activity};

/// Checks if the activity/object contained/referenced in the activity actually belongs to the author of the activity
async fn verify_ownership(state: ArcState, activity: Activity) -> Result<Activity, Error> {
    // It's fine if the objects or activities don't match in this case
    if activity.r#type == "Announce" || activity.r#type == "Follow" {
        return Ok(activity);
    }

    let identity_match = match activity.object {
        ObjectField::Actor(ref actor) => actor.id == activity.actor,
        ObjectField::Object(ref object) => object.attributed_to == activity.actor,
        ObjectField::Url(ref url) => {
            let entity = fetcher::fetch_any(&state, url).await?;
            entity.is_owned_by(activity.actor.as_str())
        }
    };

    identity_match.then_some(activity).ok_or(Error::Unauthorized)
}

/// Inbox payload extractor
///
/// This extractor also runs additional checks about whether this request is actually valid
pub struct InboxPayload(pub Activity);

#[async_trait]
impl<B> FromRequest<B> for InboxPayload
where
    B: HttpBody + Send + Sync,
    B::Data: Send,
    B::Error: StdError + Send + Sync + 'static,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(activity) = Json::<Activity>::from_request(req)
            .await
            .map_err(IntoResponse::into_response)?;

        let state = req
            .extensions()
            .get::<ArcState>()
            .expect("[Bug] State missing in request extensions");

        let (remote_actor, _remote_actor_db) = fetcher::fetch_actor(state, &activity.actor).await?;

        crypto::request::verify(
            req.method().as_str().to_string(),
            req.uri().path().to_string(),
            req.uri().query().map(ToString::to_string),
            req.headers().clone(),
            remote_actor.public_key.public_key_pem,
        )
        .await?
        .then_some(())
        .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

        let activity = verify_ownership(Arc::clone(state), activity).await?;
        Ok(Self(activity))
    }
}

/// Inbox handler
pub async fn inbox(
    Extension(state): Extension<ArcState>,
    InboxPayload(activity): InboxPayload,
) -> Result<impl IntoResponse, Error> {
    match_handler! {
        (state, activity);

        Accept,
        Announce,
        Create,
        Delete,
        Follow,
        Like,
        Reject,
        Undo,
        Update
    }
}
