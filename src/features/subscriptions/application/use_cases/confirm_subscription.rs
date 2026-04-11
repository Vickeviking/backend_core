use anyhow::Context;

use crate::features::subscriptions::application::dto::ConfirmSubscriptionCommand;
use crate::features::subscriptions::application::ports::SubscriptionRepository;
use crate::features::subscriptions::domain::SubscriptionToken;

#[derive(thiserror::Error, Debug)]
pub enum ConfirmSubscriptionUseCaseError {
    #[error("Subscribe token did not exist")]
    NonExistingToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub async fn execute_confirm_subscription<R>(
    command: ConfirmSubscriptionCommand,
    repository: &R,
) -> Result<(), ConfirmSubscriptionUseCaseError>
where
    R: SubscriptionRepository,
{
    let subscription_token = SubscriptionToken::from(command.subscription_token);

    let subscriber_id = repository
        .get_subscriber_id_from_token(&subscription_token)
        .await
        .context("Failed to retrieve subscriber_id from subscription token")?
        .ok_or(ConfirmSubscriptionUseCaseError::NonExistingToken)?;

    repository
        .confirm_subscriber(subscriber_id)
        .await
        .context("Failed to mark subscriber as confirmed")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::features::subscriptions::application::dto::ConfirmSubscriptionCommand;
    use crate::features::subscriptions::application::ports::SubscriptionRepository;
    use crate::features::subscriptions::application::use_cases::{
        ConfirmSubscriptionUseCaseError, execute_confirm_subscription,
    };
    use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};

    struct MissingTokenRepository;

    impl SubscriptionRepository for MissingTokenRepository {
        async fn save_pending_subscription(
            &self,
            _new_subscriber: &NewSubscriber,
            _subscription_token: &SubscriptionToken,
        ) -> Result<(), anyhow::Error> {
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

    #[tokio::test]
    async fn confirm_returns_non_existing_token_when_lookup_fails() {
        let command = ConfirmSubscriptionCommand {
            subscription_token: "missing".to_string(),
        };

        let error = execute_confirm_subscription(command, &MissingTokenRepository)
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            ConfirmSubscriptionUseCaseError::NonExistingToken
        ));
    }
}
