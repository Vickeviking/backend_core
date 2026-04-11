use crate::features::subscriptions::application::dto::SubscribeCommand;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub name: String,
}

impl From<SubscribeFormData> for SubscribeCommand {
    fn from(value: SubscribeFormData) -> Self {
        Self {
            email: value.email,
            name: value.name,
        }
    }
}
