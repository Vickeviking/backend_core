use crate::features::newsletter::application::dto::{DeliveryTask, PublishNewsletterIssueCommand};
use crate::features::newsletter::domain::NewsletterIssue;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait NewsletterPublicationRepository {
    async fn insert_newsletter_issue(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        command: &PublishNewsletterIssueCommand,
    ) -> Result<Uuid, sqlx::Error>;

    async fn enqueue_delivery_tasks(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        newsletter_issue_id: Uuid,
    ) -> Result<(), sqlx::Error>;
}

#[allow(async_fn_in_trait)]
pub trait NewsletterDeliveryRepository {
    async fn dequeue_delivery_task(&self) -> Result<Option<DeliveryTask>, anyhow::Error>;

    async fn delete_delivery_task(&self, task: DeliveryTask) -> Result<(), anyhow::Error>;

    async fn get_issue(&self, issue_id: Uuid) -> Result<NewsletterIssue, anyhow::Error>;
}
