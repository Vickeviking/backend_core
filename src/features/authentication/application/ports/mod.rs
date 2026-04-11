use secrecy::SecretString;
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait AuthenticationRepository {
    async fn get_stored_credentials(
        &self,
        username: &str,
    ) -> Result<Option<(Uuid, SecretString)>, anyhow::Error>;

    async fn get_username(&self, user_id: Uuid) -> Result<String, anyhow::Error>;

    async fn update_password_hash(
        &self,
        user_id: Uuid,
        password_hash: SecretString,
    ) -> Result<(), anyhow::Error>;
}
