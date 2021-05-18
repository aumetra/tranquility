#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

//! Ratelimit library based on governor for warp
//!
//! Example:
//! ```rust
//! use {tranquility_ratelimit::{Configuration, ratelimit}, warp::Filter};
//!
//! // This filter can only be accessed 100 times per hour per IP address
//! let filter =
//!     warp::any()
//!         .map(|| "Hello (ratelimited) world!")
//!         .with(ratelimit!(from_config: Configuration::new()).unwrap());
//!
//! warp::serve(filter).run(([127, 0, 0, 1], 8080))/* .await */;
//! ```
//!

use {
    governor::{
        clock::{Clock, DefaultClock},
        state::keyed::DefaultKeyedStateStore,
        Quota, RateLimiter,
    },
    std::{convert::TryInto, net::SocketAddr, num::NonZeroU32, sync::Arc, time::Duration},
    warp::{
        http::{header::RETRY_AFTER, StatusCode},
        reject::Reject,
        Filter, Rejection, Reply,
    },
};

type ArcRatelimiter =
    Arc<RateLimiter<SocketAddr, DefaultKeyedStateStore<SocketAddr>, DefaultClock>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The ratelimit burst quota is zero")]
    BurstQuotaIsZero,

    #[error("The ratelimit duration is zero")]
    DurationIsZero,
}

#[derive(Debug)]
struct WaitUntil(u64);

impl Reject for WaitUntil {}

/// Ratelimit configuration  
///
/// This defaults to 50 requests per hour  
///
#[derive(Clone, Copy)]
pub struct Configuration {
    active: bool,
    period: Duration,
    burst_quota: u32,
}

impl Configuration {
    /// Create a new instance of `Configuration`
    pub fn new() -> Self {
        Self {
            active: true,
            // 50 request per hour per IP
            period: Duration::from_secs(3600),
            burst_quota: 50,
        }
    }

    /// Set the ratelimiter active  
    /// (only really useful for user controlled configuration)  
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;

        self
    }

    /// The quota resets every given duration  
    pub fn period(mut self, period: Duration) -> Self {
        self.period = period;

        self
    }

    /// The quota for every duration  
    pub fn burst_quota(mut self, burst_quota: u32) -> Self {
        self.burst_quota = burst_quota;

        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
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

/// Check if the IP is ratelimited. If it is, reject the request
async fn check_ratelimit(
    rate_limiter: ArcRatelimiter,
    ip_address: Option<SocketAddr>,
) -> Result<(), Rejection> {
    let ip_address = ip_address.unwrap();

    rate_limiter.check_key(&ip_address).map_err(|not_until| {
        let wait_duration = not_until
            .wait_time_from(DefaultClock::default().now())
            .as_secs();

        WaitUntil(wait_duration).into()
    })
}

/// Use this as your recover function for the filter  
///
/// This is required because we throw a rejection to skip the execution of any filters after the ratelimiter  
/// (I currently don't know any other way to achieve this with warp)
///
#[doc(hidden)]
pub async fn __recover_fn(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(wait_until) = rejection.find::<WaitUntil>() {
        let wait_until = wait_until.0;

        let http_reply = warp::reply::with_header(
            format!("Too many requests. Retry in {} seconds", wait_until),
            RETRY_AFTER,
            wait_until,
        );
        Ok(warp::reply::with_status(
            http_reply,
            StatusCode::TOO_MANY_REQUESTS,
        ))
    } else {
        Err(rejection)
    }
}

/// Filter that ratelimits all logic that follows this filter
pub fn ratelimit(
    config: Configuration,
) -> Result<impl Filter<Extract = (), Error = Rejection> + Clone, Error> {
    let active = config.active;
    let rate_limiter = Arc::new(RateLimiter::keyed(config.try_into()?));

    let filter = warp::addr::remote()
        .and_then(move |ip_address| {
            let rate_limiter = Arc::clone(&rate_limiter);

            async move {
                if active {
                    check_ratelimit(rate_limiter, ip_address).await
                } else {
                    Ok(())
                }
            }
        })
        .untuple_one();

    Ok(filter)
}

/// Use this on with `.with`
/// Like: `warp::any().with(ratelimit!(Config::default())?)`
///
#[macro_export]
macro_rules! ratelimit {
    // Create a `WrapFn` using `ratelimit!(fn_from_config: )`
    // Input: [ratelimit configuration]
    (from_config: $config:expr) => {{
        $crate::ratelimit!(fn_from_config: $config).map($crate::warp::wrap_fn)
    }};

    // Create a function that can be used for the `warp::wrap_fn` function
    // Input: [ratelimit configuration]
    (fn_from_config: $config:expr) => {{
        $crate::ratelimit($config)
            .map(|ratelimit_filter| $crate::ratelimit!(fn_from_filter: ratelimit_filter))
    }};

    // Create a `warp::wrap_fn` compatible function that uses the `__recover_fn` for recovering
    // Input: [warp filter]
    (fn_from_filter: $ratelimit_filter:expr) => {{
        let ratelimit_filter = $ratelimit_filter;

        move |filter| {
            ratelimit_filter
                .clone()
                .and(filter)
                .recover($crate::__recover_fn)
        }
    }};

    // Create a `WrapFn` from the given filter using `ratelimit!(fn_from_filter: )`
    // Input: [warp filter]
    (from_filter: $ratelimit_filter:expr) => {{
        warp::wrap_fn($crate::ratelimit!(fn_from_filter: $ratelimit_filter))
    }};
}

#[doc(hidden)]
pub use warp;
