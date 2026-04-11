use crate::features::newsletter::application::use_cases::{
    ExecutionOutcome, execute_delivery_task,
};
use crate::infrastructure::config::Settings;
use crate::infrastructure::db::get_connection_pool;
use crate::shared::email_client::EmailClient;
use sqlx::PgPool;
use std::time::Duration;

pub struct Worker {
    pool: PgPool,
    email_client: EmailClient,
}

impl Worker {
    pub fn new(pool: PgPool, email_client: EmailClient) -> Self {
        Self { pool, email_client }
    }

    pub async fn run_until_stopped(self) -> Result<(), anyhow::Error> {
        worker_loop(self.pool, self.email_client).await
    }
}

pub fn build_worker(configuration: &Settings) -> Worker {
    let connection_pool = get_connection_pool(&configuration.database);
    let email_client = configuration.email_client.clone().client();
    Worker::new(connection_pool, email_client)
}

pub async fn run_worker_until_stopped(configuration: Settings) -> Result<(), anyhow::Error> {
    build_worker(&configuration).run_until_stopped().await
}

async fn worker_loop(pool: PgPool, email_client: EmailClient) -> Result<(), anyhow::Error> {
    loop {
        match execute_delivery_task(&pool, &email_client).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {}
        }
    }
}

pub async fn try_execute_task(
    pool: &PgPool,
    email_client: &EmailClient,
) -> Result<ExecutionOutcome, anyhow::Error> {
    execute_delivery_task(pool, email_client).await
}
