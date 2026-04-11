use super::TestApp;
use backend_core::features::newsletter::application::use_cases::ExecutionOutcome;
use backend_core::features::newsletter::infrastructure::worker::try_execute_task;
use sqlx::Row;

pub async fn dispatch_all_pending_emails(app: &TestApp) -> usize {
    run_delivery_worker_until_empty(app, 64).await
}

pub async fn run_delivery_worker_until_empty(app: &TestApp, max_iterations: usize) -> usize {
    let mut completed_tasks = 0;

    for _ in 0..max_iterations {
        match try_execute_task(&app.db_pool, &app.email_client)
            .await
            .expect("Failed to execute delivery task.")
        {
            ExecutionOutcome::TaskCompleted => completed_tasks += 1,
            ExecutionOutcome::EmptyQueue => return completed_tasks,
        }
    }

    panic!(
        "Reached the maximum number of worker iterations ({max_iterations}) before the queue was drained."
    );
}

pub async fn queued_delivery_count(app: &TestApp) -> i64 {
    let queued = sqlx::query(
        r#"
        SELECT COUNT(*) AS count
        FROM issue_delivery_queue
        "#,
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to query issue_delivery_queue.");

    queued.get::<i64, _>("count")
}

pub async fn delivered_email_count(app: &TestApp) -> usize {
    app.email_server
        .received_requests()
        .await
        .expect("Failed to inspect email requests.")
        .len()
}
