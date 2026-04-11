use crate::features::newsletter::application::dto::PublishNewsletterIssueCommand;
use crate::features::newsletter::application::ports::NewsletterPublicationRepository;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(skip_all)]
pub async fn execute_publish_issue<R>(
    command: &PublishNewsletterIssueCommand,
    transaction: &mut Transaction<'_, Postgres>,
    repository: &R,
) -> Result<Uuid, anyhow::Error>
where
    R: NewsletterPublicationRepository,
{
    let issue_id = repository
        .insert_newsletter_issue(transaction, command)
        .await
        .context("Failed to store newsletter issue details")?;
    repository
        .enqueue_delivery_tasks(transaction, issue_id)
        .await
        .context("Failed to enqueue delivery tasks")?;
    Ok(issue_id)
}
