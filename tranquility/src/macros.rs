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
