use crate::features::subscriptions::domain::value_objects::{SubscriberEmail, SubscriberName};

#[derive(Debug, Clone)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn try_from_parts(name: String, email: String) -> Result<Self, String> {
        let name = SubscriberName::parse(name)?;
        let email = SubscriberEmail::parse(email)?;

        Ok(Self { email, name })
    }
}
