use crate::support::{log_in, log_in_test_user, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    log_in_test_user(&app).await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 400);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], "password_mismatch");
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    log_in_test_user(&app).await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 400);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], "invalid_current_password");
}

#[tokio::test]
async fn changing_password_works() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    log_in_test_user(&app).await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 204);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 204);

    log_in(&app, &app.test_user.username, &new_password).await;

    let response = app.get_auth_session().await;
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["authenticated"], true);
    assert_eq!(body["username"], app.test_user.username);
}
