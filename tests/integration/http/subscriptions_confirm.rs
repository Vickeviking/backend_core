use crate::support::{create_unconfirmed_subscriber, spawn_app};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
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

    assert_eq!(confirmation_links.html.path(), "/subscriptions/confirm");

    let token = app.subscription_token_from_link(&confirmation_links.html);
    let response = app.get_subscription_confirm(&token).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscriptions_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/api/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    let app = spawn_app().await;
    let confirmation_links = create_unconfirmed_subscriber(&app).await;
    let token = app.subscription_token_from_link(&confirmation_links.html);

    app.get_subscription_confirm(&token)
        .await
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.status, "confirmed");
}
