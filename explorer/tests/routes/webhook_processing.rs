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
    let token = create_jwt("my_ultra_secure_secret");

    for network in networks {
        let test_data = get_test_data(&network);

        for payload in test_data {
            indexer_mock.send_webhook(&payload, &token).await;
            sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

            test_app.check_entity(payload.data, &network).await;
        }
    }
}

pub(crate) fn create_jwt(secret: &str) -> String {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let claims = hapi_explorer::routes::jwt_auth::TokenClaims {
        sub: "indexer".to_string(),
        iat: 1,
        exp: 10000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to generate JWT")
}