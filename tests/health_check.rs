use reqwest::StatusCode;

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let response = reqwest::get("http://localhost:8080/api/health_check")
        .await
        .expect("Failed to make a request");
    assert_eq!(response.content_length(), Some(0));
    assert_eq!(response.status(), StatusCode::OK);
}

pub fn spawn_app() {
    // let server = dojora::run().expect("Failed to bind address");

    // tokio::spawn(server);
}
