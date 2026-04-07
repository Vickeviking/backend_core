use crate::routes::error_chain_fmt;
use actix_web::{HttpResponse, ResponseError, http::StatusCode, web};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

//TODO: Implement similar error handling as in routes/subscriptions.rs, check dev.md for guidance
//under error handling

// TODO: jobba vidare och implementera felet vidare
#[derive(thiserror::Error)]
pub enum SubscribeConfirmError {
    #[error("Subscribe token did not exist")]
    NonExistingToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscribeConfirmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeConfirmError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscribeConfirmError::NonExistingToken => HttpResponse::new(StatusCode::UNAUTHORIZED),
            SubscribeConfirmError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SubscribeConfirmError> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Falied to retrieve subscriber_id form subscription token")?
        .ok_or(SubscribeConfirmError::NonExistingToken)?;

    confirm_subscriber(&pool, id)
        .await
        .context("Failed to mark subscriber as confirmed")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
     WHERE subscription_token = $1",
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:#?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
