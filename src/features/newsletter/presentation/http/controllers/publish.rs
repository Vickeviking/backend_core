use crate::features::newsletter::application::use_cases::execute_publish_issue;
use crate::features::newsletter::presentation::http::requests::PublishNewsletterFormData;
use crate::infrastructure::auth::UserId;
use crate::shared::http::{e400, e500};
use crate::shared::idempotency::{IdempotencyKey, NextAction, save_response, try_processing};
use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::PgPool;

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip_all,
    fields(user_id=%&*user_id)
)]
pub async fn publish_newsletter(
    form: web::Json<PublishNewsletterFormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let (command, idempotency_key) = form.0.into_parts();
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    let mut transaction = match try_processing(pool.get_ref(), &idempotency_key, *user_id)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => return Ok(saved_response),
    };

    execute_publish_issue(&command, &mut transaction, pool.get_ref())
        .await
        .map_err(e500)?;

    let response = HttpResponse::Accepted().json(json!({
        "status": "accepted"
    }));
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(e500)?;
    Ok(response)
}
