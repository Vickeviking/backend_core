use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct PendingSubscriptionModel {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) name: String,
    pub(crate) subscribed_at: DateTime<Utc>,
    pub(crate) status: &'static str,
}

pub(crate) struct SubscriptionTokenModel {
    pub(crate) subscription_token: String,
    pub(crate) subscriber_id: Uuid,
}

#[derive(sqlx::FromRow)]
pub(crate) struct SubscriberIdByTokenRow {
    pub(crate) subscriber_id: Uuid,
}
