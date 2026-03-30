use backend_core::configuration::{DatabaseSettings, get_configuration};
use backend_core::startup::{Application, get_connection_pool};
use backend_core::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

/// Initializes tracing once for the whole integration test process.
/// `TEST_LOG` switches output to stdout; otherwise logs are discarded to keep
/// normal test runs quiet.
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

/// Handles exposed to each integration test after booting the application.
/// `address` is the HTTP base URL, while `db_pool` lets tests inspect the
/// database state directly when asserting side effects.
/// `email_server` allows interception of emails directed to i.e PostMark
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscription(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request")
    }
}

/// Boots a fresh application instance for one integration test.
/// The helper initializes tracing once, provisions an isolated database,
/// starts the server on a random port, and returns the handles tests need.
pub async fn spawn_app() -> TestApp {
    // Executed once by first invocation
    Lazy::force(&TRACING);

    // Launch a mock server to stand in for Postmark's API
    let email_server = MockServer::start().await;

    // randomize configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Different database for each testcase
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as the background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    // retrieve port before spawning application
    let address = format!("http://127.0.0.1:{}", application.port());

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
    }
}

/// Creates a brand-new database for the current test and runs migrations.
/// Returning the pool gives the test a handle into the same isolated database
/// the application will use after startup.
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
