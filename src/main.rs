use backend_core::startup::run;
use backend_core::telemetry::init_subscriber;
use backend_core::{configuration::get_configuration, telemetry::get_subscriber};

use std::net::TcpListener;

use sqlx::PgPool;

use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("backend_core".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
