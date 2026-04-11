use crate::features::authentication::application::dto::ChangePasswordCommand;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct ChangePasswordFormData {
    current_password: SecretString,
    new_password: SecretString,
    new_password_check: SecretString,
}

impl ChangePasswordFormData {
    pub fn into_command(self) -> ChangePasswordCommand {
        ChangePasswordCommand {
            current_password: self.current_password,
            new_password: self.new_password,
            new_password_check: self.new_password_check,
        }
    }
}
