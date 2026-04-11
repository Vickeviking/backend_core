use crate::support::{log_in_test_user, spawn_app};

#[tokio::test]
async fn session_reports_unauthenticated_without_login() {
    let app = spawn_app().await;

    let response = app.get_auth_session().await;
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["authenticated"], false);
}

#[tokio::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;

    log_in_test_user(&app).await;

    let response = app.get_auth_session().await;
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["authenticated"], true);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 204);

    let response = app.get_auth_session().await;
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["authenticated"], false);
}
