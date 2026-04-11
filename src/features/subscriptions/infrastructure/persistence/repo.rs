use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::features::subscriptions::application::ports::SubscriptionRepository;
use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};
use crate::features::subscriptions::infrastructure::persistence::mappers::{
    subscriber_id_from_row, to_pending_subscription_model, to_subscription_token_model,
};
use crate::features::subscriptions::infrastructure::persistence::models::SubscriberIdByTokenRow;

pub struct SqlxSubscriptionRepository {
    pool: PgPool,
}

impl SqlxSubscriptionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SubscriptionRepository for SqlxSubscriptionRepository {
    async fn save_pending_subscription(
        &self,
        new_subscriber: &NewSubscriber,
        subscription_token: &SubscriptionToken,
    ) -> Result<(), anyhow::Error> {
        let pending_subscription = to_pending_subscription_model(new_subscriber);
        let token_record = to_subscription_token_model(subscription_token, pending_subscription.id);

        let mut transaction = self
            .pool
            .begin()
            .await
            .context("Failed to acquire a Postgres connection from the pool")?;

        sqlx::query(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at, status)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(pending_subscription.id)
        .bind(pending_subscription.email)
        .bind(pending_subscription.name)
        .bind(pending_subscription.subscribed_at)
        .bind(pending_subscription.status)
        .execute(&mut *transaction)
        .await
        .context("Failed to insert new subscriber in the database")?;

        sqlx::query(
            r#"
            INSERT INTO subscription_tokens (subscription_token, subscriber_id)
            VALUES ($1, $2)
            "#,
        )
        .bind(token_record.subscription_token)
        .bind(token_record.subscriber_id)
        .execute(&mut *transaction)
        .await
        .context("Failed to store the confirmation token for a new subscriber")?;

        transaction
            .commit()
            .await
            .context("Failed to commit SQL transaction to store a new subscriber")?;

        Ok(())
    }

    async fn get_subscriber_id_from_token(
        &self,
        subscription_token: &SubscriptionToken,
    ) -> Result<Option<Uuid>, anyhow::Error> {
        let result = sqlx::query_as::<_, SubscriberIdByTokenRow>(
            r#"
            SELECT subscriber_id
            FROM subscription_tokens
            WHERE subscription_token = $1
            "#,
        )
        .bind(subscription_token.as_ref())
        .fetch_optional(&self.pool)
        .await
        .context("Failed to retrieve subscriber_id from subscription token")?;

        Ok(result.map(subscriber_id_from_row))
    }

    async fn confirm_subscriber(&self, subscriber_id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query(r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#)
            .bind(subscriber_id)
            .execute(&self.pool)
            .await
            .context("Failed to mark subscriber as confirmed")?;

        Ok(())
    }
}
