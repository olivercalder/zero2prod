#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn app.");
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> std::io::Result<()> {
    // Calling zero2prod::run().await here would wait for run to return (which never happens) and
    // thus hang forever and never run the test code.
    todo!()
}
