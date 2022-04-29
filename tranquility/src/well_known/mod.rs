use axum::Router;

pub fn routes() -> Router {
    let router = Router::new()
        .merge(nodeinfo::routes())
        .merge(webfinger::routes());

    Router::new().nest("/.well-known", router)
}

pub mod nodeinfo;
pub mod webfinger;
