use super::{ConfirmationLinks, TestApp};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

pub fn newsletter_request_body() -> serde_json::Value {
    serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    })
}

pub async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();
    let body = serde_json::json!({
        "name": name,
        "email": email
    });

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscription(&body)
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request)
}

pub async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    let token = app.subscription_token_from_link(&confirmation_link.html);

    app.get_subscription_confirm(&token)
        .await
        .error_for_status()
        .unwrap();
}

pub async fn insert_confirmed_subscriber(app: &TestApp) {
    sqlx::query(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, now(), 'confirmed')
        "#,
    )
    .bind(uuid::Uuid::new_v4())
    .bind("confirmed@example.com")
    .bind("Confirmed Subscriber")
    .execute(&app.db_pool)
    .await
    .expect("Failed to store confirmed subscriber.");
}
