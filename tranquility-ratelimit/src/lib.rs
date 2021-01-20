#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

//! Ratelimit library based on governor for warp
//!
//! Example:
//! ```rust
//! use {tranquility_ratelimit::Configuration, warp::Filter};
//!
//! let filter = warp::any().map(|| "Hello (ratelimited) world!");
//! // This filter can only be accessed 100 times per hour per IP address
//! let assembled_filter = tranquility_ratelimit::ratelimit!(filter => filter, config => Configuration::new()).unwrap();
//!
//! warp::serve(assembled_filter).run(([127, 0, 0, 1], 8080))/* .await */;
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
/// Example:
/// ```rust
/// use {tranquility_ratelimit::Configuration, warp::Filter};
///
/// let ratelimit = tranquility_ratelimit::ratelimit(Configuration::default()).unwrap();
/// let filter = warp::any().map(|| "Hello world!");
///
/// let assembled_filter = ratelimit.and(filter).recover(tranquility_ratelimit::recover_fn);
/// ```
///
pub async fn recover_fn(rejection: Rejection) -> Result<impl Reply, Rejection> {
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

/// Convenience macro  
///
/// This macro assembles a ratelimited filter complete with config and recover function  
#[macro_export]
macro_rules! ratelimit {
    (filter => $filter:ident, config => $config:expr) => {{
        $crate::ratelimit($config)
            .map(move |filter| $crate::custom_ratelimit!(filter => $filter, ratelimit_filter => filter))
    }};
}

#[macro_export]
macro_rules! custom_ratelimit {
    (filter => $filter:ident, ratelimit_filter => $ratelimit_filter:ident) => {{
        $ratelimit_filter.and($filter).recover($crate::recover_fn)
    }};
}
