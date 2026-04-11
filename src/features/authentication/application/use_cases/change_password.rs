use crate::features::authentication::application::ports::AuthenticationRepository;
use crate::infrastructure::logging::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[tracing::instrument(name = "Change password", skip(password, repository))]
pub async fn execute_change_password<R>(
    user_id: Uuid,
    password: SecretString,
    repository: &R,
) -> Result<(), anyhow::Error>
where
    R: AuthenticationRepository,
{
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;
    repository
        .update_password_hash(user_id, password_hash)
        .await?;
    Ok(())
}

fn compute_password_hash(password: SecretString) -> Result<SecretString, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(SecretString::new(password_hash.into()))
}
