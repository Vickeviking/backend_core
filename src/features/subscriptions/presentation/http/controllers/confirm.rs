use actix_web::{HttpResponse, ResponseError, web};
use serde_json::json;
use sqlx::PgPool;

use crate::features::subscriptions::application::use_cases::{
    ConfirmSubscriptionUseCaseError, execute_confirm_subscription,
};
use crate::features::subscriptions::domain::errors::error_chain_fmt;
use crate::features::subscriptions::infrastructure::persistence::SqlxSubscriptionRepository;
use crate::features::subscriptions::presentation::http::requests::ConfirmSubscriptionParameters;

#[derive(thiserror::Error)]
pub enum SubscribeConfirmHttpError {
    #[error("Subscribe token did not exist")]
    NonExistingToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscribeConfirmHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeConfirmHttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscribeConfirmHttpError::NonExistingToken => {
                HttpResponse::Unauthorized().json(json!({
                    "code": "invalid_token"
                }))
            }
            SubscribeConfirmHttpError::UnexpectedError(_) => HttpResponse::InternalServerError()
                .json(json!({
                    "code": "internal_error"
                })),
        }
    }
}

impl From<ConfirmSubscriptionUseCaseError> for SubscribeConfirmHttpError {
    fn from(value: ConfirmSubscriptionUseCaseError) -> Self {
        match value {
            ConfirmSubscriptionUseCaseError::NonExistingToken => Self::NonExistingToken,
            ConfirmSubscriptionUseCaseError::UnexpectedError(error) => Self::UnexpectedError(error),
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<ConfirmSubscriptionParameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SubscribeConfirmHttpError> {
    let command = parameters.0.into();
    let repository = SqlxSubscriptionRepository::new(pool.get_ref().clone());

    execute_confirm_subscription(command, &repository)
        .await
        .map_err(SubscribeConfirmHttpError::from)?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "confirmed"
    })))
}
