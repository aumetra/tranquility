use {
    crate::{
        config::{
            Configuration, ConfigurationInstance, ConfigurationRatelimit, ConfigurationServer,
            ConfigurationTls,
        },
        state::State,
    },
    sqlx::PgPool,
    std::env,
};

fn test_config() -> Configuration {
    Configuration {
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

async fn init_db() -> PgPool {
    let conn_url = env::var("TEST_DB_URL").unwrap();

    let conn_pool = PgPool::connect(&conn_url).await.unwrap();
    crate::database::migrate(&conn_pool).await.ok();

    conn_pool
}

async fn test_state() -> State {
    let config = test_config();
    let db_pool = init_db().await;

    State::new_arcless(config, db_pool)
}

mod follow_activity;
mod oauth;
mod register;
