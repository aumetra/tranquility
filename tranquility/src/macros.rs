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

/// Format UUIDs in a unified way
#[macro_export]
macro_rules! format_uuid {
    ($uuid:expr) => {{
        $uuid.to_simple_ref().to_string()
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

/// Something like `map_err!(Err::<(), ()>(()))` expands to
///
/// ```rust
/// Err::<(), ()>(()).map_err(crate::error::Error::from)
/// ```
#[macro_export]
macro_rules! map_err {
    ($op:expr) => {{
        $op.map_err(crate::error::Error::from)
    }};
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

/// Limit the body size of a filter
#[macro_export(local_inner_macros)]
macro_rules! limit_body_size {
    // Use the default maximum body size
    ($filter:expr) => {{
        limit_body_size!($filter, crate::consts::MAX_BODY_SIZE)
    }};
    // Use the user-defined maximum body size
    ($filter:expr, $limit:expr) => {{
        warp::body::content_length_limit($limit).and($filter)
    }};
    // Multiply the user-defined maximum body size with the MB in bytes constant
    ($filter:expr, $limit:ident MB) => {{
        limit_body_size!($filter, $limit * crate::consts::MB_BYTES)
    }};
}
