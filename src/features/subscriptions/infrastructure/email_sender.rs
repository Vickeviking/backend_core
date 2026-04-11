use anyhow::Context;

use crate::features::subscriptions::application::ports::SubscriptionEmailSender;
use crate::features::subscriptions::domain::{NewSubscriber, SubscriptionToken};
use crate::shared::email_client::EmailClient;

pub struct ConfirmationEmailSender<'a> {
    email_client: &'a EmailClient,
}

impl<'a> ConfirmationEmailSender<'a> {
    pub fn new(email_client: &'a EmailClient) -> Self {
        Self { email_client }
    }
}

impl SubscriptionEmailSender for ConfirmationEmailSender<'_> {
    async fn send_confirmation_email(
        &self,
        new_subscriber: &NewSubscriber,
        base_url: &str,
        subscription_token: &SubscriptionToken,
    ) -> Result<(), anyhow::Error> {
        let confirmation_link =
            format!("{base_url}/subscriptions/confirm?subscription_token={subscription_token}",);

        let plain_body = format!(
            "Welcome to our newsletter!\nVisit {confirmation_link} to confirm your subscription.",
        );

        let html_body = format!(
            "Welcome to our newsletter!<br />Click <a href=\"{confirmation_link}\">here</a> to confirm your subscription.",
        );

        self.email_client
            .send_email(&new_subscriber.email, "Welcome!", &html_body, &plain_body)
            .await
            .context("Failed to send the confirmation email")?;

        Ok(())
    }
}
