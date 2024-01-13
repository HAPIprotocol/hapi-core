use {
    hapi_indexer::get_id_from_jwt,
    serde_json::json,
    tokio::time::{sleep, Duration},
};

use crate::helpers::{create_jwt, RequestSender, TestApp, WAITING_INTERVAL};

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
    let indexer_mock = RequestSender::new(test_app.server_addr.to_owned());
    let indexers_url = format!("{}/indexer", test_app.server_addr);
    let payload = &json!({"cursor": "Block: 12345"});

    for (network, _) in &test_app.networks {
        //create indexer
        let token = test_app.create_indexer(network).await;
        sleep(Duration::from_millis(WAITING_INTERVAL)).await;

        // heartbeat indexer
        let id = get_id_from_jwt(&token).expect("Failed to get id from jwt");
        let heartbeat_url = format!("{}/indexer/{}/heartbeat", &test_app.server_addr, id);

        indexer_mock
            .send(&heartbeat_url, &json!({"cursor": "Block: 12345"}), &token)
            .await
            .unwrap();
    }

    let token = &create_jwt("invalid_token");
    let id = get_id_from_jwt(&token).expect("Failed to get id from jwt");
    let heartbeat_url = format!("{}/indexer/{}/heartbeat", &test_app.server_addr, id);
    // heartbeat indexer with wrong token
    assert!(indexer_mock
        .send(&heartbeat_url, &payload, &token)
        .await
        .is_err());

    // get indexers
    let response = indexer_mock.get(&indexers_url).await.unwrap();

    // check count of indexers
    let indexers: Vec<serde_json::Value> = response["data"].as_array().unwrap().to_vec();
    assert_eq!(indexers.len(), 3);
}
