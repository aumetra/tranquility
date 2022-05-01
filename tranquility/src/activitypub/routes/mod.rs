use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CollectionQuery {
    last_id: Option<Uuid>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/users/:id", get(users::users))
        .route("/users/:id/followers", get(followers::followers))
        .route("/users/:id/following", get(following::following))
        .route("/users/:id/inbox", post(inbox::inbox))
        .route("/users/:id/outbox", get(outbox::outbox))
        .route("/objects/:id", get(objects::objects))
}

pub mod followers;
pub mod following;
pub mod inbox;
pub mod objects;
pub mod outbox;
pub mod users;
