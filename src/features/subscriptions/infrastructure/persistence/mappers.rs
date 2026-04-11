use chrono::Utc;
use uuid::Uuid;

use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};
use crate::features::subscriptions::infrastructure::persistence::models::{
    PendingSubscriptionModel, SubscriberIdByTokenRow, SubscriptionTokenModel,
};

pub(crate) fn to_pending_subscription_model(
    new_subscriber: &NewSubscriber,
) -> PendingSubscriptionModel {
    PendingSubscriptionModel {
        id: Uuid::new_v4(),
        email: new_subscriber.email.as_ref().to_string(),
        name: new_subscriber.name.as_ref().to_string(),
        subscribed_at: Utc::now(),
        status: "pending_confirmation",
    }
}

pub(crate) fn to_subscription_token_model(
    subscription_token: &SubscriptionToken,
    subscriber_id: Uuid,
) -> SubscriptionTokenModel {
    SubscriptionTokenModel {
        subscription_token: subscription_token.as_ref().to_string(),
        subscriber_id,
    }
}

pub(crate) fn subscriber_id_from_row(row: SubscriberIdByTokenRow) -> Uuid {
    row.subscriber_id
}
