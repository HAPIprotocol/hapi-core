use crate::helpers::{RequestSender, TestApp};

#[tokio::test]
async fn cors_test() {
    let origins = vec![
        "http://allowed-origin1.com".to_string(),
        "http://allowed-origin2.com".to_string(),
    ];

    let test_app = TestApp::start(Some(origins.clone())).await;
    let client = RequestSender::new(test_app.server_addr.to_owned());

    for host in origins {
        let response = client
            .web_client
            .get(format!("{}/health", test_app.server_addr))
            .header("Origin", host.clone())
            .send()
            .await
            .expect("Failed to send request");

        assert!(response
            .headers()
            .contains_key("Access-Control-Allow-Origin"));

        let cors_header = response
            .headers()
            .get("Access-Control-Allow-Origin")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(cors_header, host);
    }

    let response = client
        .web_client
        .get(format!("{}/health", test_app.server_addr))
        .header("Origin", "http://forbidden-origin.com")
        .send()
        .await
        .expect("Failed to send request");

    assert!(!response
        .headers()
        .contains_key("Access-Control-Allow-Origin"));
}
