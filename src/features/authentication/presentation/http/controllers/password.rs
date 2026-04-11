use crate::features::authentication::application::use_cases::{
    execute_change_password, execute_get_username, execute_validate_credentials,
};
use crate::features::authentication::domain::AuthError;
use crate::features::authentication::presentation::http::requests::ChangePasswordFormData;
use crate::infrastructure::auth::UserId;
use crate::shared::http::e500;
use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::PgPool;

pub async fn change_password(
    form: web::Json<ChangePasswordFormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let command = form.0.into_command();

    if let Err(message) = command.ensure_new_passwords_match() {
        tracing::info!(error = %message, "Password update rejected due to mismatch");
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": "password_mismatch"
        })));
    }

    let username = execute_get_username(*user_id, pool.get_ref())
        .await
        .map_err(e500)?;

    let credentials = crate::features::authentication::domain::Credentials {
        username,
        password: command.current_password,
    };

    if let Err(e) = execute_validate_credentials(credentials, pool.get_ref()).await {
        return match e {
            AuthError::InvalidCredentials(_) => Ok(HttpResponse::BadRequest().json(json!({
                "code": "invalid_current_password"
            }))),
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    execute_change_password(*user_id, command.new_password, pool.get_ref())
        .await
        .map_err(e500)?;
    Ok(HttpResponse::NoContent().finish())
}
