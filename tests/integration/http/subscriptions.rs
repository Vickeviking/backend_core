use crate::support::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscriber_fails_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com"
    });

    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let response = app.post_subscription(&body).await;

    assert_eq!(response.status().as_u16(), 500);
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_validation() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app.post_subscription(&body).await;
    assert_eq!(response.status().as_u16(), 202);
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscription(&body).await;
    assert_eq!(response.status().as_u16(), 202);

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscription(&body).await;

    assert_eq!(202, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscription(&body).await;
    assert_eq!(response.status().as_u16(), 202);

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({"name": "", "email": "ursula_le_guin@gmail.com"}),
            "empty name",
        ),
        (
            serde_json::json!({"name": "le guin", "email": ""}),
            "empty email",
        ),
        (
            serde_json::json!({"name": "le guin", "email": "definitely-not-an-email"}),
            "invalid email",
        ),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscription(&body).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        (serde_json::json!({"name": "le guin"}), "missing the email"),
        (
            serde_json::json!({"email": "ursula_le_guin@gmail.com"}),
            "missing the name",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscription(&invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
