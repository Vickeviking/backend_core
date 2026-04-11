use crate::features::authentication::domain::Credentials;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    username: String,
    password: SecretString,
}

impl From<LoginFormData> for Credentials {
    fn from(value: LoginFormData) -> Self {
        Self {
            username: value.username,
            password: value.password,
        }
    }
}
