use crate::helpers::{RequestSender, TestApp};

#[tokio::test]
async fn health_check_test() {
    let test_app = TestApp::start().await;
    let client = RequestSender::new(test_app.server_addr.to_owned());

    client
        .get("health")
        .await
        .expect("Failed to get health check");
}
