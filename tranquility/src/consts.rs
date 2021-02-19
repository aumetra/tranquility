pub mod activitypub {
    pub const ACTIVITIES_PER_PAGE: i64 = 10;
}

pub mod cors {
    pub const API_ALLOWED_METHODS: &[&str] = &["post", "put", "delete", "get", "patch", "options"];
    pub const GENERAL_ALLOWED_METHODS: &[&str] = &["get"];
    pub const OAUTH_TOKEN_ALLOWED_METHODS: &[&str] = &["post"];
}

pub mod crypto {
    pub const KEY_SIZE: usize = 2048;
    pub const TOKEN_LENGTH: usize = 40;
}

pub mod daemon {
    use std::time::Duration;

    pub const DELETE_INTERVAL: Duration = Duration::from_secs(60);
}

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("GIT_BRANCH"),
    "-",
    env!("GIT_COMMIT")
);
