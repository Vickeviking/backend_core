use crate::features::authentication::application::dto::LoginCommand;
use crate::features::authentication::application::ports::AuthenticationRepository;
use crate::features::authentication::domain::AuthError;
use crate::infrastructure::logging::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[tracing::instrument(name = "Validate credentials", skip(credentials, repository))]
pub async fn execute_validate_credentials<R>(
    credentials: LoginCommand,
    repository: &R,
) -> Result<Uuid, AuthError>
where
    R: AuthenticationRepository,
{
    let mut user_id = None;
    let mut expected_password_hash = SecretString::new(
        "$argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .into(),
    );

    if let Some((stored_user_id, stored_password_hash)) = repository
        .get_stored_credentials(&credentials.username)
        .await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}
