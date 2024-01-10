use tokio::time::{sleep, Duration};

use crate::helpers::{create_jwt, IndexerMock, TestApp, WAITING_INTERVAL};
use hapi_core::HapiCoreNetwork;

/*
Test cases:
 - create indexer
 - heartbeat indexer
 - heartbeat indexer with wrong token
 - get indexers
 - check count of indexers
 */
#[tokio::test]
async fn indexer_processing_test() {
    let test_app = TestApp::start().await;
    let indexer_mock = IndexerMock::new(&test_app.server_addr);
    let networks = vec![
        HapiCoreNetwork::Ethereum,
        HapiCoreNetwork::Solana,
        HapiCoreNetwork::Near,
    ];

    for network in networks {
        //create indexer
        let token = TestApp::create_indexer(network).await;
        sleep(Duration::from_millis(WAITING_INTERVAL)).await;

        // heartbeat indexer
        let response = indexer_mock.send_heartbeat(&token).await;
        assert!(response.status().is_success());
    }

    // heartbeat indexer with wrong token
    assert!(!indexer_mock
        .send_heartbeat(&create_jwt("invalid_token"))
        .await
        .status()
        .is_success());

    // get indexers
    let response = indexer_mock.get_indexers().await;
    assert!(response.status().is_success());

    // check count of indexers
    let value: serde_json::Value = response.json().await.unwrap();
    let indexers: Vec<serde_json::Value> = value["data"].as_array().unwrap().to_vec();
    assert_eq!(indexers.len(), 3);
}
