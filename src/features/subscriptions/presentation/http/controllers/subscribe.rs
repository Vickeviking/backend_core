use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError, web};
use serde_json::json;
use sqlx::PgPool;

use crate::features::subscriptions::application::use_cases::{
    SubscribeUseCaseError, execute_subscribe,
};
use crate::features::subscriptions::domain::errors::error_chain_fmt;
use crate::features::subscriptions::infrastructure::email_sender::ConfirmationEmailSender;
use crate::features::subscriptions::infrastructure::persistence::SqlxSubscriptionRepository;
use crate::features::subscriptions::presentation::http::requests::SubscribeFormData;
use crate::shared::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;

#[derive(thiserror::Error)]
pub enum SubscribeHttpError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscribeHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeHttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscribeHttpError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeHttpError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            SubscribeHttpError::ValidationError(message) => {
                HttpResponse::BadRequest().json(json!({
                    "code": "invalid_input",
                    "message": message,
                }))
            }
            SubscribeHttpError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "code": "internal_error"
                }))
            }
        }
    }
}

impl From<SubscribeUseCaseError> for SubscribeHttpError {
    fn from(value: SubscribeUseCaseError) -> Self {
        match value {
            SubscribeUseCaseError::ValidationError(error) => Self::ValidationError(error),
            SubscribeUseCaseError::UnexpectedError(error) => Self::UnexpectedError(error),
        }
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Json<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, SubscribeHttpError> {
    let command = form.0.into();
    let repository = SqlxSubscriptionRepository::new(pool.get_ref().clone());
    let email_sender = ConfirmationEmailSender::new(email_client.get_ref());

    execute_subscribe(command, &repository, &email_sender, &base_url.0)
        .await
        .map_err(SubscribeHttpError::from)?;

    Ok(HttpResponse::Accepted().json(json!({
        "status": "accepted"
    })))
}
