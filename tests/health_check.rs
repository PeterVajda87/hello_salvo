#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn app");

    let client = reqwest::Client::new();

    let response = client
        .get("http://showdown.monster:5800/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    hello_salvo::run();
    Ok(())
}
