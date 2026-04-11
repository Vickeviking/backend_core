use uuid::Uuid;

use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};

#[allow(async_fn_in_trait)]
pub trait SubscriptionRepository {
    async fn save_pending_subscription(
        &self,
        new_subscriber: &NewSubscriber,
        subscription_token: &SubscriptionToken,
    ) -> Result<(), anyhow::Error>;

    async fn get_subscriber_id_from_token(
        &self,
        subscription_token: &SubscriptionToken,
    ) -> Result<Option<Uuid>, anyhow::Error>;

    async fn confirm_subscriber(&self, subscriber_id: Uuid) -> Result<(), anyhow::Error>;
}

#[allow(async_fn_in_trait)]
pub trait SubscriptionEmailSender {
    async fn send_confirmation_email(
        &self,
        new_subscriber: &NewSubscriber,
        base_url: &str,
        subscription_token: &SubscriptionToken,
    ) -> Result<(), anyhow::Error>;
}
