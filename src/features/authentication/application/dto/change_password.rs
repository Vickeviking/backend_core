use secrecy::{ExposeSecret, SecretString};

pub struct ChangePasswordCommand {
    pub current_password: SecretString,
    pub new_password: SecretString,
    pub new_password_check: SecretString,
}

impl ChangePasswordCommand {
    pub fn ensure_new_passwords_match(&self) -> Result<(), &'static str> {
        if self.new_password.expose_secret() == self.new_password_check.expose_secret() {
            Ok(())
        } else {
            Err("You entered two different new passwords - the field values must match.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChangePasswordCommand;
    use secrecy::SecretString;

    #[test]
    fn rejects_mismatched_new_passwords() {
        let command = ChangePasswordCommand {
            current_password: SecretString::new("current-password".into()),
            new_password: SecretString::new("new-password".into()),
            new_password_check: SecretString::new("other-password".into()),
        };

        assert!(command.ensure_new_passwords_match().is_err());
    }
}
