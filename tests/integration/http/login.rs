use crate::support::{log_in_test_user, spawn_app};

#[tokio::test]
async fn invalid_credentials_return_401() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], "invalid_credentials");

    let response = app.get_auth_session().await;
    assert_eq!(response.status().as_u16(), 200);
    let session_body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(session_body["authenticated"], false);
}

#[tokio::test]
async fn login_success_sets_authenticated_session() {
    let app = spawn_app().await;

    log_in_test_user(&app).await;

    let response = app.get_auth_session().await;
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["authenticated"], true);
    assert_eq!(body["username"], app.test_user.username);
}
