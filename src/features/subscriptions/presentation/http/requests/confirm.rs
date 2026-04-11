use crate::features::subscriptions::application::dto::ConfirmSubscriptionCommand;

#[derive(serde::Deserialize)]
pub struct ConfirmSubscriptionParameters {
    pub subscription_token: String,
}

impl From<ConfirmSubscriptionParameters> for ConfirmSubscriptionCommand {
    fn from(value: ConfirmSubscriptionParameters) -> Self {
        Self {
            subscription_token: value.subscription_token,
        }
    }
}
