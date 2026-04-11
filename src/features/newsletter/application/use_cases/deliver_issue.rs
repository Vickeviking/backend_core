use crate::features::newsletter::application::ports::NewsletterDeliveryRepository;
use crate::features::subscriptions::domain::SubscriberEmail;
use crate::shared::email_client::EmailClient;
use tracing::{Span, field::display};

pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

#[tracing::instrument(
    skip_all,
    fields(
        newsletter_issue_id=tracing::field::Empty,
        subscriber_email=tracing::field::Empty
    ),
    err
)]
pub async fn execute_delivery_task<R>(
    repository: &R,
    email_client: &EmailClient,
) -> Result<ExecutionOutcome, anyhow::Error>
where
    R: NewsletterDeliveryRepository,
{
    let Some(task) = repository.dequeue_delivery_task().await? else {
        return Ok(ExecutionOutcome::EmptyQueue);
    };

    Span::current()
        .record("newsletter_issue_id", display(task.newsletter_issue_id))
        .record("subscriber_email", display(&task.subscriber_email));

    match SubscriberEmail::parse(task.subscriber_email.clone()) {
        Ok(email) => {
            let issue = repository.get_issue(task.newsletter_issue_id).await?;
            if let Err(e) = email_client
                .send_email(
                    &email,
                    &issue.title,
                    &issue.html_content,
                    &issue.text_content,
                )
                .await
            {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Failed to deliver issue to a confirmed subscriber. Skipping.",
                );
            }
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Skipping a confirmed subscriber. Their stored contact details are invalid",
            );
        }
    }

    repository.delete_delivery_task(task).await?;
    Ok(ExecutionOutcome::TaskCompleted)
}
