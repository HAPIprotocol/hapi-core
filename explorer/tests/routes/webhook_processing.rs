use tokio::time::{sleep, Duration};

use crate::helpers::{create_jwt, get_test_data, IndexerMock, TestApp, WAITING_INTERVAL};
use hapi_core::HapiCoreNetwork;

#[tokio::test]
async fn webhook_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = IndexerMock::new(&test_app.server_addr);
    let networks = vec![
        HapiCoreNetwork::Ethereum,
        HapiCoreNetwork::Solana,
        HapiCoreNetwork::Near,
    ];
    let token = create_jwt("my_ultra_secure_secret");

    for network in networks {
        let test_data = get_test_data(&network);

        for payload in test_data {
            indexer_mock.send_webhook(&payload, &token).await;
            sleep(Duration::from_millis(WAITING_INTERVAL)).await;

            test_app.check_entity(payload.data, &network).await;
        }
    }
}
