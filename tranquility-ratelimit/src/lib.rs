#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate
)]

//!
//! Ratelimit library based on governor for axum
//!

#[macro_use]
extern crate tracing;

use axum::{
    extract::{ConnectInfo, FromRequest, RequestParts},
    http::{
        header::{HeaderName, RETRY_AFTER},
        Request, StatusCode,
    },
    response::{IntoResponse, Response},
};
use futures_util::future::{BoxFuture, FutureExt};
use governor::{
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter,
};
use std::{
    convert::TryInto,
    error::Error as StdError,
    net::{IpAddr, SocketAddr},
    num::NonZeroU32,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tower_layer::Layer;
use tower_service::Service;

static PROXY_HEADER: HeaderName = HeaderName::from_static("x-forwarded-for");

type ArcRatelimiter = Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>;
type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The ratelimit burst quota is zero")]
    BurstQuotaIsZero,

    #[error("The ratelimit duration is zero")]
    DurationIsZero,
}

#[derive(Debug)]
struct WaitUntil(u64);

impl IntoResponse for WaitUntil {
    fn into_response(self) -> Response {
        let wait_until = self.0;

        let http_reply = (
            [(RETRY_AFTER, wait_until)],
            format!("Too many requests. Retry in {} seconds", wait_until),
        );
        (StatusCode::TOO_MANY_REQUESTS, http_reply).into_response()
    }
}

/// Ratelimit configuration  
///
/// This defaults to 50 requests per hour  
///
#[derive(Clone, Copy)]
pub struct Configuration {
    active: bool,
    trust_proxy: bool,
    period: Duration,
    burst_quota: u32,
}

impl Configuration {
    /// Create a new instance of `Configuration`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ratelimiter active  
    /// (only really useful for user controlled configuration)  
    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// The quota resets every given duration  
    #[must_use]
    pub fn period(mut self, period: Duration) -> Self {
        self.period = period;
        self
    }

    /// Trust the value of the `X-Forwarded-For` header
    #[must_use]
    pub fn trust_proxy(mut self, trust_proxy: bool) -> Self {
        self.trust_proxy = trust_proxy;
        self
    }

    /// The quota for every duration  
    #[must_use]
    pub fn burst_quota(mut self, burst_quota: u32) -> Self {
        self.burst_quota = burst_quota;
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            active: true,
            // Trust the `X-Forwarded-For` header
            trust_proxy: true,
            // 50 request per hour per IP
            period: Duration::from_secs(3600),
            burst_quota: 50,
        }
    }
}

impl TryInto<Quota> for Configuration {
    type Error = Error;

    fn try_into(self) -> Result<Quota, Self::Error> {
        Ok(Quota::with_period(self.period)
            .ok_or(Error::DurationIsZero)?
            .allow_burst(NonZeroU32::new(self.burst_quota).ok_or(Error::BurstQuotaIsZero)?))
    }
}

/// Ratelimit service for axum servers
#[derive(Clone)]
pub struct RatelimitService<S> {
    inner: S,
    config: Configuration,
    limiter: ArcRatelimiter,
}

impl<S> RatelimitService<S> {
    /// Construct a new ratelimit service
    pub fn new(
        inner: S,
        config: Configuration,
    ) -> Result<Self, <Configuration as TryInto<Quota>>::Error> {
        Ok(Self {
            inner,
            config,
            limiter: Arc::new(RateLimiter::keyed(config.try_into()?)),
        })
    }
}

impl<S, B> Service<Request<B>> for RatelimitService<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Error: Into<BoxError>,
    S::Future: Send,
    B: Send + 'static,
{
    type Error = BoxError;
    type Response = Response;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        let config = self.config;
        let limiter = Arc::clone(&self.limiter);

        async move {
            if config.active {
                let mut req_parts = RequestParts::new(req);
                let ip_addr = if config.trust_proxy {
                    // Read the value of the `X-Forwarded-For` header and attempt to parse it into an IP address
                    // We ignore all errors here since they are most likely the result of a reverse proxy misconfiguration
                    // Just logging a generic message out should be enough
                    if let Some(Ok(Ok(ip_addr))) = req_parts
                        .headers()
                        .get(&PROXY_HEADER)
                        .map(|header_value| header_value.to_str().map(FromStr::from_str))
                    {
                        ip_addr
                    } else {
                        error!("Failed to parse the value of the {PROXY_HEADER} header into an IP address. Check your reverse proxy configuration!");
                        return Ok(StatusCode::BAD_REQUEST.into_response());
                    }
                } else {
                    let ConnectInfo(socket_addr) =
                        ConnectInfo::<SocketAddr>::from_request(&mut req_parts).await?;

                    socket_addr.ip()
                };

                if let Err(not_until) = limiter.check_key(&ip_addr) {
                    let wait_until = not_until
                        .wait_time_from(DefaultClock::default().now())
                        .as_secs();

                    return Ok(WaitUntil(wait_until).into_response());
                }

                inner.call(req_parts.try_into_request().unwrap()).await.map_err(Into::into)
            } else {
                inner.call(req).await.map_err(Into::into)
            }
        }
        .boxed()
    }
}

/// Layer for the ratelimit service
#[derive(Clone, Copy)]
pub struct RatelimitLayer {
    config: Configuration,
}

impl RatelimitLayer {
    /// Construct a new ratelimit layer
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }
}

impl<S> Layer<S> for RatelimitLayer {
    type Service = RatelimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RatelimitService::new(inner, self.config)
            .expect("Broken configuration passed to ratelimit layer")
    }
}
