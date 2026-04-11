use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct DeliveryTask {
    pub transaction: Transaction<'static, Postgres>,
    pub newsletter_issue_id: Uuid,
    pub subscriber_email: String,
}
