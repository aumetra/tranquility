use crate::consts::USER_AGENT;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{Form as AxumForm, FromRequest, RequestParts},
    Json,
};
use futures_util::FutureExt;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{error::Error as StdError, future::Future};
use tokio::sync::oneshot;

pub static HTTP_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().user_agent(USER_AGENT).build().unwrap());

/// Specialised form that deserialises both, JSON and URL-encoded form data
pub struct Form<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Form<T>
where
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: StdError + Send + Sync + 'static,
    T: DeserializeOwned,
{
    type Rejection = <AxumForm<T> as FromRequest<B>>::Rejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Ok(val) = Json::<T>::from_request(req).await {
            Ok(Self(val))
        } else {
            let val = AxumForm::from_request(req).await?;
            Ok(Self(val))
        }
    }
}

/// Run any CPU intensive tasks (RSA key generation, password hashing, etc.) via this function
pub fn cpu_intensive_task<F, T>(func: F) -> impl Future<Output = T> + Send + Sync + 'static
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (sender, receiver) = oneshot::channel();

    rayon::spawn(move || {
        let span = info_span!(
            "CPU intensive task",
            worker_id = rayon::current_thread_index().unwrap()
        );
        let _enter_guard = span.enter();

        let result = func();

        if sender.send(result).is_err() {
            warn!("Couldn't send result back to async task");
        }
    });

    receiver.map(Result::unwrap)
}

pub mod mention;
