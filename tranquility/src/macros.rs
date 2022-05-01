/// Try fetching an something that can be turned into an `Entity` via the given methods  
/// If the fetch succeeds, the function returns with the success value  
/// If it doesn't, the error gets logged and the function continues  
#[macro_export]
macro_rules! attempt_fetch {
    ($state:ident, $url:ident, [$($func:ident),+]) => {{
        $(
            match $func($state, $url).await {
                Ok(val) => return Ok(val.into()),
                Err(err) => tracing::debug!(error = ?err, "Couldn't fetch entity"),
            }
        )+
    }};
}

/// Creates a macro to emulate a constant that can be used with the `concat!` macro
///
/// This creates an macro with the name that expands to just the literal as well as an constant with the same value for type enforcement
///
/// ```
/// r#const!(TEST: &str = "test");
/// ```
///
/// expands to
///
/// ```
/// macro_rules! TEST {
///     () => {
///         "test"
///     };
/// }
///
/// #[doc(hidden)]
/// #[allow(dead_code)]
/// const TEST: &str = TEST!();
/// ```
#[macro_export]
macro_rules! r#const {
    ($ident:ident: $type:ty = $val:literal) => {
        macro_rules! $ident {
            () => {
                $val
            };
        }

        #[doc(hidden)]
        #[allow(dead_code)]
        const $ident: $type = $ident!();
    };
}

/// Format UUIDs in a unified way
#[macro_export]
macro_rules! format_uuid {
    ($uuid:expr) => {{
        $uuid.as_simple().to_string()
    }};
}

/// This macro is intended for enums like `Entity` whose arms have the same name as their containing type
///
/// Something like `impl_from!(Entity; Activity)` expands to
/// ```
/// impl From<Activity> for Entity {
///     fn from(value: Activity) -> Entity {
///         Self::Activity(value)
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_from {
    ($enum:ty; $($type:ident),+) => {
        $(
            impl From<$type> for $enum {
                fn from(value: $type) -> Self {
                    Self::$type(value)
                }
            }
        )+
    }
}

/// This macro is intended for enums like `Entity` whose arms have the same name as their containing type
///
/// Something like `impl_into!(Entity; Activity)` expands to
/// ```
/// impl Entity {
///     pub fn into_activity(self) -> Option<Activity> {
///         match self {
///             Self::Activity(val) => Some(val),
///             _ => None,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_into {
    ($enum:ty; $($type:ident),+) => {
        paste::paste! {
            impl $enum {
                $(
                    #[allow(dead_code)]
                    pub fn [<into_ $type:lower>](self) -> Option<$type> {
                        match self {
                            Self::$type(val) => Some(val),
                            _ => None,
                        }
                    }
                )+
            }
        }
    }
}

/// This macro is intended for enums like `Entity` that can contain multiple different ActivityPub entities
///
/// Every ActivityPub entity has some kind of reference to their author in form of the actor ID but the fields that contain those IDs can be named differently
///
/// For example:
/// - `Actor` => `id`
/// - `Activity` => `actor`
/// - `Object` => `attributedTo`
///
/// Something like `impl_is_owned_by!(Entity; (Activity, actor))` expands to
/// ```
/// impl Entity {
///     pub fn is_owned_by(&self, actor_id: &str) -> bool {
///         match self {
///             Self::Activity(val) => val.actor == actor_id,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_is_owned_by {
    ($enum:ty; $(($branch:ident, $field:ident)),+) => {
        impl $enum {
            pub fn is_owned_by(&self, actor_id: &str) -> bool {
                match self {
                    $(
                        Self::$branch(val) => val.$field == actor_id,
                    )+
                }
            }
        }
    }
}

/// This expands to a match expression with arms like these
///
/// ```rust
/// "Accept" => crate::activitypub::handler::accept::handle(&state, activity).await
/// ```
#[macro_export]
macro_rules! match_handler {
    {
        ($state:ident, $activity:ident);

        $($type:ident),+
    } => {
        paste::paste! {
            match $activity.r#type.as_str() {
                $(
                    stringify!($type) =>
                        crate::activitypub::handler::[<$type:lower>]::handle(&$state, $activity).await,
                )+
                _ => Err(crate::error::Error::UnknownActivity),
            }
        }
    }
}

/// Construct a new ratelimit layer that's compatible with axum
#[macro_export]
macro_rules! ratelimit_layer {
    ($active:expr, $use_forwarded_header:expr, $reqs_per_hour:expr $(,)+) => {{
        let config = ::tranquility_ratelimit::Configuration::default()
            .active($active)
            .burst_quota($reqs_per_hour)
            .trust_proxy($use_forwarded_header);

        ::tower::ServiceBuilder::new()
            .layer(::axum::error_handling::HandleErrorLayer::new(|err| async move {
                error!(error = %err, "Ratelimiting call failed");

                ::http::StatusCode::INTERNAL_SERVER_ERROR
            }))
            .layer(::tranquility_ratelimit::RatelimitLayer::new(config))
    }};
}

/// Compiles the regex and saves it into a lazy (so that it doesn't have to be recompiled for every usage)
///
/// This is comparable with the regex macro example from the OnceCell docs except that we use lazy instead of initialising a OnceCell ourselves.
/// This doesn't make a difference though because a lazy uses a OnceCell internally anyway
///
/// Examples:
/// ```
/// regex!(TEST = "^.+$")
/// ```
///
/// expands to
///
/// ```
/// static TEST: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
///     regex::Regex::new("^.+$").expect("Regex compilation failed")
/// });
/// ```
///
/// ---
///
/// ```
/// let test_regex = regex!("^.+$");
/// ```
///
/// expands to
///
/// ```
/// let test_regex = {
///     static REGEX: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
///         regex::Regex::new("^.+$").expect("Regex compilation failed")
///     });
///     
///     &*REGEX
/// };
/// ```
#[macro_export]
macro_rules! regex {
    ($ident:ident = $regex:ident) => {
        static $ident: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new($regex).expect("Regex compilation failed")
        });
    };
    ($regex:ident) => {{
        regex!(REGEX = $regex);

        &*REGEX
    }};
}
