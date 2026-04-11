use crate::features::authentication::application::ports::AuthenticationRepository;
use uuid::Uuid;

#[tracing::instrument(name = "Get username", skip(repository))]
pub async fn execute_get_username<R>(user_id: Uuid, repository: &R) -> Result<String, anyhow::Error>
where
    R: AuthenticationRepository,
{
    repository.get_username(user_id).await
}
