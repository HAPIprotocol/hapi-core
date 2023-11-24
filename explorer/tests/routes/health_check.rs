use crate::helpers::TestApp;

#[tokio::test]
async fn metrics_check_works() {
    let test_app = TestApp::start().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/metrics", &test_app.server_addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    println!("{:?}", response.text().await);
    // assert_eq!(Some(0), response.content_length());
}
