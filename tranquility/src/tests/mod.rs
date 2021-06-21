use {
    crate::{
        activitypub::FollowActivity,
        config::{
            Configuration, ConfigurationEmail, ConfigurationInstance, ConfigurationRatelimit,
            ConfigurationServer, ConfigurationTls,
        },
        state::State,
    },
    sqlx::PgPool,
    std::env,
};

const FOLLOW_ACTIVITY: &str = r#"
{
    "cc": ["https://www.w3.org/ns/activitystreams#Public"],
    "id": "https://a.example.com/activities/8dcc256a-8c3f-49ee-ab22-bb51c9082260",
    "to": ["https://b.example.com/users/test"],
    "type": "Follow",
    "actor": "https://a.example.com/users/test",
    "state": "pending",
    "object": "https://b.example.com/users/test",
    "context": "https://a.example.com/contexts/9c3b4420-dd74-454b-8124-c4759b849f3a",
    "published": "2019-08-20T14:02:09.995388Z",
    "context_id": 8
}
"#;

fn test_config() -> Configuration {
    Configuration {
        email: ConfigurationEmail {
            active: false,
            server: "smtp.example.com".into(),
            starttls: false,
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

#[test]
fn decode_follow_activity() {
    let follow_activity: FollowActivity = serde_json::from_str(FOLLOW_ACTIVITY).unwrap();

    assert_eq!(follow_activity.activity.r#type, "Follow");
    assert!(!follow_activity.approved);
}

mod nodeinfo;
mod register;
