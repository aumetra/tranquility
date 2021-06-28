use {
    crate::{
        config::{
            Configuration, ConfigurationEmail, ConfigurationInstance, ConfigurationRatelimit,
            ConfigurationServer, ConfigurationTls,
        },
        state::State,
    },
    sqlx::PgPool,
    std::env,
};

const TEST_PASSWORD: &str = "1234567.";

/// Create a test configuration
fn test_config() -> Configuration {
    Configuration {
        email: ConfigurationEmail {
            active: false,
            server: "smtp.example.com".into(),
            starttls: false,
            email: "noreply@example.com".into(),
            username: "tranquility".into(),
            password: "tranquility-acct-password".into(),
        },
        instance: ConfigurationInstance {
            closed_registrations: false,
            domain: "tranquility.example.com".into(),
            description: "Tranquility instance".into(),
            character_limit: 1024,
            upload_limit: 4096,
            moderators: Vec::new(),
        },
        ratelimit: ConfigurationRatelimit {
            active: false,
            authentication_quota: 1,
            registration_quota: 1,
        },
        server: ConfigurationServer {
            database_url: String::new(),
            interface: "127.0.0.1".into(),
            port: 8080,
        },
        tls: ConfigurationTls {
            serve_tls_directly: false,
            certificate: String::new(),
            secret_key: String::new(),
        },
    }
}

/// Migrate the database
async fn init_db() -> PgPool {
    let conn_url = env::var("TEST_DB_URL").unwrap();

    let conn_pool = PgPool::connect(&conn_url).await.unwrap();
    crate::database::migrate(&conn_pool).await.ok();

    conn_pool
}

/// Migrate the database and construct a state (without an `Arc` so the values can be adjusted as needed)
async fn test_state() -> State {
    let config = test_config();
    let db_pool = init_db().await;

    State::new_arcless(config, db_pool)
}

mod follow_activity;
mod oauth;
mod register;

#[cfg(feature = "mastodon-api")]
mod mastodon;
