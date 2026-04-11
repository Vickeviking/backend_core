use anyhow::Context;

use crate::features::subscriptions::application::dto::SubscribeCommand;
use crate::features::subscriptions::application::ports::{
    SubscriptionEmailSender, SubscriptionRepository,
};
use crate::features::subscriptions::domain::{NewSubscriber, generate_subscription_token};

#[derive(thiserror::Error, Debug)]
pub enum SubscribeUseCaseError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub async fn execute_subscribe<R, E>(
    command: SubscribeCommand,
    repository: &R,
    email_sender: &E,
    base_url: &str,
) -> Result<(), SubscribeUseCaseError>
where
    R: SubscriptionRepository,
    E: SubscriptionEmailSender,
{
    let new_subscriber = NewSubscriber::try_from_parts(command.name, command.email)
        .map_err(SubscribeUseCaseError::ValidationError)?;
    let subscription_token = generate_subscription_token();

    repository
        .save_pending_subscription(&new_subscriber, &subscription_token)
        .await
        .context("Failed to persist the pending subscriber and confirmation token")?;

    email_sender
        .send_confirmation_email(&new_subscriber, base_url, &subscription_token)
        .await
        .context("Failed to send a confirmation email")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use uuid::Uuid;

    use crate::features::subscriptions::application::dto::SubscribeCommand;
    use crate::features::subscriptions::application::ports::{
        SubscriptionEmailSender, SubscriptionRepository,
    };
    use crate::features::subscriptions::application::use_cases::execute_subscribe;
    use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};

    #[derive(Default)]
    struct FakeRepository {
        saved_tokens: Mutex<Vec<String>>,
    }

    impl SubscriptionRepository for FakeRepository {
        async fn save_pending_subscription(
            &self,
            _new_subscriber: &NewSubscriber,
            subscription_token: &SubscriptionToken,
        ) -> Result<(), anyhow::Error> {
            self.saved_tokens
                .lock()
                .unwrap()
                .push(subscription_token.as_ref().to_string());
            Ok(())
        }

        async fn get_subscriber_id_from_token(
            &self,
            _subscription_token: &SubscriptionToken,
        ) -> Result<Option<Uuid>, anyhow::Error> {
            Ok(None)
        }

        async fn confirm_subscriber(&self, _subscriber_id: Uuid) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeEmailSender {
        sent_tokens: Mutex<Vec<String>>,
    }

    impl SubscriptionEmailSender for FakeEmailSender {
        async fn send_confirmation_email(
            &self,
            _new_subscriber: &NewSubscriber,
            _base_url: &str,
            subscription_token: &SubscriptionToken,
        ) -> Result<(), anyhow::Error> {
            self.sent_tokens
                .lock()
                .unwrap()
                .push(subscription_token.as_ref().to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn subscribe_persists_and_sends_the_same_confirmation_token() {
        let repository = FakeRepository::default();
        let email_sender = FakeEmailSender::default();
        let command = SubscribeCommand {
            email: "ursula@example.com".to_string(),
            name: "Ursula Le Guin".to_string(),
        };

        execute_subscribe(command, &repository, &email_sender, "http://127.0.0.1:8000")
            .await
            .unwrap();

        let saved_tokens = repository.saved_tokens.lock().unwrap().clone();
        let sent_tokens = email_sender.sent_tokens.lock().unwrap().clone();

        assert_eq!(saved_tokens.len(), 1);
        assert_eq!(sent_tokens.len(), 1);
        assert_eq!(saved_tokens, sent_tokens);
    }
}
