use crate::support::{
    create_confirmed_subscriber, create_unconfirmed_subscriber, dispatch_all_pending_emails,
    log_in_test_user, newsletter_request_body, spawn_app,
};
use std::time::Duration;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn you_must_be_logged_in_to_publish_newsletters() {
    let app = spawn_app().await;

    let response = app
        .post_publish_newsletter(&newsletter_request_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let response = app
        .post_publish_newsletter(&newsletter_request_body())
        .await;
    assert_eq!(response.status().as_u16(), 202);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "accepted");
    dispatch_all_pending_emails(&app).await;
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app
        .post_publish_newsletter(&newsletter_request_body())
        .await;
    assert_eq!(response.status().as_u16(), 202);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "accepted");
    dispatch_all_pending_emails(&app).await;
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = newsletter_request_body();
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_eq!(response.status().as_u16(), 202);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "accepted");

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_eq!(response.status().as_u16(), 202);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "accepted");

    dispatch_all_pending_emails(&app).await;
}

#[tokio::test]
async fn concurrent_form_submission_is_handled_gracefully() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = newsletter_request_body();
    let response1 = app.post_publish_newsletter(&newsletter_request_body);
    let response2 = app.post_publish_newsletter(&newsletter_request_body);
    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );
    dispatch_all_pending_emails(&app).await;
}
