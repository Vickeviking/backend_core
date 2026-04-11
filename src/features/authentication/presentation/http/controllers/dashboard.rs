use crate::features::authentication::application::use_cases::execute_get_username;
use crate::shared::http::e500;
use crate::shared::session_state::TypedSession;
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

#[derive(serde::Serialize)]
struct SessionResponse {
    authenticated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

pub async fn auth_session(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let Some(user_id) = session.get_user_id().map_err(e500)? else {
        return Ok(HttpResponse::Ok().json(SessionResponse {
            authenticated: false,
            username: None,
        }));
    };

    let username = execute_get_username(user_id, pool.get_ref())
        .await
        .map_err(e500)?;
    Ok(HttpResponse::Ok().json(SessionResponse {
        authenticated: true,
        username: Some(username),
    }))
}
