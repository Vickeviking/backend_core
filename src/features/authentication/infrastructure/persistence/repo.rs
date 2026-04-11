use crate::features::authentication::application::ports::AuthenticationRepository;
use anyhow::Context;
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

impl AuthenticationRepository for PgPool {
    async fn get_stored_credentials(
        &self,
        username: &str,
    ) -> Result<Option<(Uuid, SecretString)>, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, password_hash
            FROM users
            WHERE username = $1
            "#,
            username,
        )
        .fetch_optional(self)
        .await
        .context("Failed to perform a query to retrieve stored credentials.")?;

        Ok(row.map(|row| (row.user_id, SecretString::new(row.password_hash.into()))))
    }

    async fn get_username(&self, user_id: Uuid) -> Result<String, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT username
            FROM users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(self)
        .await
        .context("Failed to perform a query to retrieve a username.")?;
        Ok(row.username)
    }

    async fn update_password_hash(
        &self,
        user_id: Uuid,
        password_hash: SecretString,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $1
            WHERE user_id = $2
            "#,
            password_hash.expose_secret(),
            user_id
        )
        .execute(self)
        .await
        .context("Failed to change user's password in the database.")?;
        Ok(())
    }
}
