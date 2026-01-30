use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    //arange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //act
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = backend_core::run(listener).expect("Failed to bind address");

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);
    //return application address to caller

    format!("http://127.0.0.1:{}", port)
}
