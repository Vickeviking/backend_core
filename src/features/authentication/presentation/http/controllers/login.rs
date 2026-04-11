use crate::features::authentication::application::use_cases::execute_validate_credentials;
use crate::features::authentication::domain::{AuthError, Credentials};
use crate::features::authentication::presentation::http::requests::LoginFormData;
use crate::shared::http::e500;
use crate::shared::session_state::TypedSession;
use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::PgPool;

#[tracing::instrument(
    skip(form, pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Json<LoginFormData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let credentials: Credentials = form.0.into();
    tracing::Span::current().record("username", tracing::field::display(&credentials.username));

    match execute_validate_credentials(credentials, pool.get_ref()).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));
            session.renew();
            session.insert_user_id(user_id).map_err(e500)?;
            Ok(HttpResponse::NoContent().finish())
        }
        Err(AuthError::InvalidCredentials(_)) => Ok(HttpResponse::Unauthorized().json(json!({
            "code": "invalid_credentials"
        }))),
        Err(AuthError::UnexpectedError(error)) => Err(e500(error)),
    }
}
