use crate::helpers::{get_test_data, IndexerMock, TestApp, WAITING_TIMESTAMP};
use {
    hapi_core::HapiCoreNetwork,
    tokio::time::{sleep, Duration},
};

#[tokio::test]
async fn webhoock_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = IndexerMock::new(&test_app.server_addr);
    let networks = vec![
        HapiCoreNetwork::Ethereum,
        HapiCoreNetwork::Solana,
        HapiCoreNetwork::Near,
    ];

    for network in networks {
        let test_data = get_test_data(&network);

        for payload in test_data {
            indexer_mock.send_webhook(&payload).await;
            sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

            test_app.check_entity(payload.data, &network).await;
        }
    }
}
