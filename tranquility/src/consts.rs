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

pub mod regex {
    use crate::r#const;

    r#const!(USERNAME_BASE: &str = r#"[\w]+"#);

    pub const USERNAME: &str = concat!("^", USERNAME_BASE!(), "$");
    // Regex101 link (for explaination of the regex): https://regex101.com/r/pyTTsW/1
    pub const MENTION: &str = concat!(
        r#"(?:^|\W)@("#,
        USERNAME_BASE!(),
        r#")(?:@([\w\.\-]+[[:alnum:]]+))?"#
    );
}

// Default to 5MB
pub const MAX_BODY_SIZE: u64 = 5 * MB_BYTES;
pub const MB_BYTES: u64 = 1024_u64.pow(2);

pub const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const PROPER_VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("GIT_BRANCH"),
    "-",
    env!("GIT_COMMIT")
);
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
