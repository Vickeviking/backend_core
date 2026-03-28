use backend_core::startup::build;
use backend_core::telemetry::init_subscriber;
use backend_core::{configuration::get_configuration, telemetry::get_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("backend_core".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let server = build(configuration).await?;
    server.await?;

    Ok(())
}
