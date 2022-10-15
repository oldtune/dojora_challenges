use std::time::Duration;

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    tokio::time::sleep(Duration::from_secs(3)).await?;
    let response = reqwest::get("http://localhost:8080/health_check").await?;
    assert_eq!(response.content_length(), Some(0));
}

pub async fn spawn_app() {
    let server = dojora::run().expect("Failed to bind address");

    tokio::spawn(server);
}
