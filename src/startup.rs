use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Creates a lazily connected Postgres pool for the configured database.
/// Production startup and test helpers both reuse this so request handlers
/// talk to the same database that was configured for that environment.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

/// Owns the bound HTTP server together with the port it listens on.
/// The binary awaits it directly, while tests read the assigned port
/// after binding to `0` and before the server is spawned.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Builds the application from [`Settings`] and binds its TCP listener.
    /// This creates the database pool and email client, then delegates to
    /// [`run`] to construct the Actix server with those dependencies.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    /// Returns the port the listener was bound to.
    /// Tests use this after requesting port `0` so they can build the base
    /// URL before the server future is handed off to Tokio.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Consumes the application and awaits the underlying Actix server.
    /// This is the main process path in production and the future tests
    /// spawn in the background once setup is complete.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// Wrapper type needed since actix-web is type-based
pub struct ApplicationBaseUrl(pub String);

/// Constructs and starts the HTTP server from pre-built infrastructure.
/// Keeping this separate from [`Application::build`] lets tests inject
/// their own listener, database pool, and email client when needed.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(health_check))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
