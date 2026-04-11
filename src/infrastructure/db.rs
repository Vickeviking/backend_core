use crate::infrastructure::config::DatabaseSettings;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Creates a lazily connected Postgres pool for the configured database.
/// Production startup and test helpers both reuse this so request handlers
/// talk to the same database that was configured for that environment.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
