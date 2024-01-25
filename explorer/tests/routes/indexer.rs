use crate::helpers::{create_jwt, RequestSender, TestApp};

/*
Test cases:
 - heartbeat indexer
 - heartbeat indexer with wrong token
 - get indexers
 - check count of indexers
 */
#[tokio::test]
async fn indexer_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = RequestSender::new(test_app.server_addr.to_owned());

    for network in &test_app.networks {
        // heartbeat indexer
        indexer_mock.send_heartbeat(&network.token).await.unwrap();
    }

    // heartbeat indexer with wrong token
    assert!(indexer_mock
        .send_heartbeat(&create_jwt("invalid_token"))
        .await
        .is_err());

    // get indexers
    let response = indexer_mock.get("indexer").await.unwrap();

    // check count of indexers
    let indexers: Vec<serde_json::Value> = response["data"].as_array().unwrap().to_vec();
    assert_eq!(indexers.len(), test_app.networks.len());
}
