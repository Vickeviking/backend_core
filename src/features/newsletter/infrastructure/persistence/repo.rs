use crate::features::newsletter::application::dto::{DeliveryTask, PublishNewsletterIssueCommand};
use crate::features::newsletter::application::ports::{
    NewsletterDeliveryRepository, NewsletterPublicationRepository,
};
use crate::features::newsletter::domain::NewsletterIssue;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

impl NewsletterPublicationRepository for PgPool {
    async fn insert_newsletter_issue(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        command: &PublishNewsletterIssueCommand,
    ) -> Result<Uuid, sqlx::Error> {
        let newsletter_issue_id = Uuid::new_v4();
        let query = sqlx::query!(
            r#"
            INSERT INTO newsletter_issues (
                newsletter_issue_id, 
                title, 
                text_content, 
                html_content,
                published_at
            )
            VALUES ($1, $2, $3, $4, now())
            "#,
            newsletter_issue_id,
            command.title,
            command.text_content,
            command.html_content
        );
        transaction.execute(query).await?;
        Ok(newsletter_issue_id)
    }

    async fn enqueue_delivery_tasks(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        newsletter_issue_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"
            INSERT INTO issue_delivery_queue (
                newsletter_issue_id, 
                subscriber_email
            )
            SELECT $1, email
            FROM subscriptions
            WHERE status = 'confirmed'
            "#,
            newsletter_issue_id,
        );
        transaction.execute(query).await?;
        Ok(())
    }
}

impl NewsletterDeliveryRepository for PgPool {
    async fn dequeue_delivery_task(&self) -> Result<Option<DeliveryTask>, anyhow::Error> {
        let mut transaction = self.begin().await?;
        let r = sqlx::query!(
            r#"
            SELECT newsletter_issue_id, subscriber_email
            FROM issue_delivery_queue
            FOR UPDATE
            SKIP LOCKED
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *transaction)
        .await?;

        Ok(r.map(|r| DeliveryTask {
            transaction,
            newsletter_issue_id: r.newsletter_issue_id,
            subscriber_email: r.subscriber_email,
        }))
    }

    async fn delete_delivery_task(&self, task: DeliveryTask) -> Result<(), anyhow::Error> {
        let DeliveryTask {
            mut transaction,
            newsletter_issue_id,
            subscriber_email,
        } = task;
        sqlx::query!(
            r#"
            DELETE FROM issue_delivery_queue
            WHERE 
                newsletter_issue_id = $1 AND
                subscriber_email = $2 
            "#,
            newsletter_issue_id,
            subscriber_email
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn get_issue(&self, issue_id: Uuid) -> Result<NewsletterIssue, anyhow::Error> {
        let issue = sqlx::query_as!(
            NewsletterIssue,
            r#"
            SELECT title, text_content, html_content
            FROM newsletter_issues
            WHERE
                newsletter_issue_id = $1
            "#,
            issue_id
        )
        .fetch_one(self)
        .await?;
        Ok(issue)
    }
}
