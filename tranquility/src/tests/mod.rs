use crate::{
    activitypub::FollowActivity,
    config::{
        Configuration, ConfigurationEmail, ConfigurationInstance, ConfigurationJaeger,
        ConfigurationRatelimit, ConfigurationServer, ConfigurationTls,
    },
    server::create_router_make_service,
    state::{ArcState, State},
};
use axum::Server;
use mime::Mime;
use sqlx::PgPool;
use std::{env, net::SocketAddr};

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
        jaeger: ConfigurationJaeger {
            active: false,
            host: "localhost".into(),
            port: 6831,
        },
        ratelimit: ConfigurationRatelimit {
            active: false,
            use_forwarded_header: false,
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

struct TestClient {
    address: SocketAddr,
    client: reqwest::Client,
}

impl TestClient {
    /// Construct a new test client
    fn new(address: SocketAddr) -> Self {
        Self {
            address,
            client: reqwest::Client::new(),
        }
    }

    fn format_url(&self, uri: &str) -> String {
        format!("http://{}{uri}", self.address)
    }

    /// Send a GET request
    async fn get(&self, uri: &str) -> reqwest::Result<reqwest::Response> {
        self.client.get(self.format_url(uri)).send().await
    }

    /// Send a POST request
    ///
    /// If `None` is passed as the content type it defaults to `application/x-www-form-urlencoded`
    async fn post<B>(
        &self,
        uri: &str,
        content_type: Option<Mime>,
        body: B,
    ) -> reqwest::Result<reqwest::Response>
    where
        B: Into<reqwest::Body>,
    {
        let content_type = content_type.unwrap_or(mime::APPLICATION_WWW_FORM_URLENCODED);

        self.client
            .post(self.format_url(uri))
            .header("Content-Type", content_type.as_ref())
            .body(body)
            .send()
            .await
    }
}

/// Start an axum server bound to a random port
///
/// # Returns
///
/// Returns a client that can send HTTP requests to the test server
fn start_test_server<S>(state: S) -> TestClient
where
    S: Into<ArcState>,
{
    let state = state.into();
    let router_service = create_router_make_service(&state);

    let server = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 0))).serve(router_service);
    let bound_address = server.local_addr();

    tokio::spawn(server);

    TestClient::new(bound_address)
}

#[test]
fn decode_follow_activity() {
    let follow_activity: FollowActivity = serde_json::from_str(FOLLOW_ACTIVITY).unwrap();

    assert_eq!(follow_activity.activity.r#type, "Follow");
    assert!(!follow_activity.approved);
}

mod nodeinfo;
mod register;
