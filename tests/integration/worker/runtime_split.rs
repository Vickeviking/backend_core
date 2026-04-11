use crate::support::{
    delivered_email_count, insert_confirmed_subscriber, log_in_test_user, newsletter_request_body,
    queued_delivery_count, run_delivery_worker_until_empty, spawn_app,
};
use std::time::Duration;

#[tokio::test]
async fn api_boots_without_starting_the_worker() {
    let app = spawn_app().await;
    insert_confirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    let response = app
        .post_publish_newsletter(&newsletter_request_body())
        .await;
    assert_eq!(response.status().as_u16(), 202);

    tokio::time::sleep(Duration::from_millis(250)).await;

    assert_eq!(queued_delivery_count(&app).await, 1);
    assert_eq!(delivered_email_count(&app).await, 0);
}

#[tokio::test]
async fn worker_processes_queued_newsletter_deliveries_on_its_own() {
    let app = spawn_app().await;
    insert_confirmed_subscriber(&app).await;
    log_in_test_user(&app).await;

    let response = app
        .post_publish_newsletter(&newsletter_request_body())
        .await;
    assert_eq!(response.status().as_u16(), 202);

    let completed_tasks = run_delivery_worker_until_empty(&app, 8).await;

    assert_eq!(completed_tasks, 1);
    assert_eq!(queued_delivery_count(&app).await, 0);
    assert_eq!(delivered_email_count(&app).await, 1);
}
