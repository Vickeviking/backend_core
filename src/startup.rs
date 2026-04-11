use crate::features::authentication::presentation::http::{
    auth_session, change_password, log_out, login,
};
use crate::features::newsletter::presentation::http::publish_newsletter;
use crate::features::subscriptions::presentation::http::{confirm, subscribe};
use crate::infrastructure::auth::reject_anonymous_users;
use crate::infrastructure::config::Settings;
pub use crate::infrastructure::db::get_connection_pool;
use crate::operational::http::health_check;
use crate::shared::email_client::EmailClient;
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

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
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let email_client = configuration.email_client.client();

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
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await?;

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
async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let hmac_secret = web::Data::new(hmac_secret);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .route("/health_check", web::get().to(health_check))
                    .route("/auth/session", web::get().to(auth_session))
                    .route("/auth/login", web::post().to(login))
                    .route("/auth/logout", web::post().to(log_out))
                    .route("/subscriptions", web::post().to(subscribe))
                    .route("/subscriptions/confirm", web::get().to(confirm))
                    .service(
                        web::scope("/admin")
                            .wrap(from_fn(reject_anonymous_users))
                            .route("/newsletters", web::post().to(publish_newsletter))
                            .route("/password", web::post().to(change_password)),
                    ),
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(hmac_secret.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
