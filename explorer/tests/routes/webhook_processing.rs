use crate::helpers::{create_jwt, get_test_data, RequestSender, TestApp, WAITING_INTERVAL};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn webhook_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = RequestSender::new(test_app.server_addr.clone());
    let token = create_jwt("my_ultra_secure_secret");

    for (network, _) in &test_app.networks {
        let test_data = get_test_data(network.to_owned());

        for payload in test_data {
            indexer_mock
                .send("events", &payload, &token)
                .await
                .expect("Failed to send event");
            sleep(Duration::from_millis(WAITING_INTERVAL)).await;

            test_app
                .check_entity(payload.data, network.to_owned())
                .await;
        }
    }
}
