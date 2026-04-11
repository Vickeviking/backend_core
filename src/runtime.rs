use crate::infrastructure::config::{Settings, get_configuration};
use crate::infrastructure::logging::{get_subscriber, init_subscriber};
use anyhow::Context;

pub fn initialize_process(process_name: &str) -> Result<Settings, anyhow::Error> {
    initialize_telemetry(process_name);
    load_configuration()
}

pub fn initialize_telemetry(process_name: &str) {
    let subscriber = get_subscriber(
        format!("backend_core_{process_name}"),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
}

pub fn load_configuration() -> Result<Settings, anyhow::Error> {
    get_configuration().context("Failed to read configuration.")
}
