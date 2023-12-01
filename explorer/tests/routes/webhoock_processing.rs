use crate::helpers::{get_test_data, IndexerMock, TestApp};

#[tokio::test]
async fn webhoock_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = IndexerMock::new(&test_app.server_addr);
    let test_data = get_test_data();

    for payload in test_data {
        indexer_mock.send_webhook(&payload).await;
        test_app.check_entity(payload.data).await;
    }
}
