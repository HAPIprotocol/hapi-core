use crate::helpers::{get_test_data, RequestSender, TestApp, WAITING_TIMESTAMP};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn webhook_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = RequestSender::new(test_app.server_addr.clone());

    for network in &test_app.networks {
        let test_data = get_test_data(network.to_owned());

        for payload in test_data {
            indexer_mock.send("events", &payload).await;
            sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

            test_app
                .check_entity(payload.data, network.to_owned())
                .await;
        }
    }
}
