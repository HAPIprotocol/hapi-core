use super::replacer;
use crate::helpers::{RequestSender, TestApp, TestData};

use {
    hapi_core::client::{entities::address::Address, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_ADDRESS_QUERY: &str = "
    query GetAddress($address: String!, $network: UUID!) {
        getAddress(address: $address, network: $network) {
            network
            address
            caseId
            reporterId
            risk
            category
            confirmations
            createdAt
            updatedAt
        }
    }
";
const GET_MANY_ADDRESSES: &str = "
    query GetManyAddresses(
        $input: AddressInput!
    ) {
        getManyAddresses(
            input: $input
        ) {
            data {
                network
                address
                caseId
                reporterId
                risk
                category
                confirmations
                createdAt
                updatedAt
            }
            total
            pageCount
        }
    }
";

impl From<PushPayload> for TestData<Address> {
    fn from(payload: PushPayload) -> Self {
        let entity = match &payload.data {
            PushData::Address(address) => address,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network: payload.network,
            indexer_id: payload.id,
        }
    }
}

fn check_address(address: &TestData<Address>, value: &Value) {
    assert_eq!(
        replacer(&value["network"]),
        address.network.to_string().to_lowercase()
    );

    let payload = &address.data;
    assert_eq!(value["address"], payload.address);
    assert_eq!(value["caseId"], payload.case_id.to_string());
    assert_eq!(value["reporterId"], payload.reporter_id.to_string());
    assert_eq!(value["risk"], payload.risk);
    assert_eq!(
        replacer(&value["category"]),
        payload.category.to_string().to_lowercase()
    );
    assert_eq!(value["confirmations"], payload.confirmations.to_string());
}

#[tokio::test]
async fn get_address_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities::<Address>(&sender, EventName::UpdateAddress, None)
        .await;

    for payload in addresses {
        let response = sender
            .send_graphql(
                GET_ADDRESS_QUERY,
                json!({
                    "address": payload.data.address,
                    "network": payload.network.to_string().to_uppercase()
                }),
            )
            .await
            .unwrap();

        let address = &response["getAddress"];
        check_address(&payload, address);
    }
}

#[tokio::test]
async fn get_many_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities::<Address>(&sender, EventName::UpdateAddress, None)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_ADDRESSES,
            json!({
            "input":
            {
              "ordering": "ASC",
              "orderingCondition": "UPDATED_AT",
            }

            }),
        )
        .await
        .unwrap();

    let addresses_response = &response["getManyAddresses"];
    assert_eq!(addresses_response["total"], addresses.len());

    for (index, address) in addresses_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let payload = addresses.get(index).expect("Invalid index");

        check_address(payload, address)
    }
}

#[tokio::test]
async fn get_filtered_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities::<Address>(&sender, EventName::UpdateAddress, None)
        .await;

    for payload in addresses {
        let response = sender
            .send_graphql(
                GET_MANY_ADDRESSES,
                json!({
                "input":
                {
                    "filtering": {
                        "reporterId": payload.data.reporter_id.to_string(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }

                }),
            )
            .await
            .unwrap();

        let addresses_response = &response["getManyAddresses"];
        assert_eq!(addresses_response["total"], 1);

        let address = addresses_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_address(&payload, address)
    }
}

#[tokio::test]
async fn get_paginated_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities::<Address>(&sender, EventName::UpdateAddress, None)
        .await;

    let payload = addresses.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_ADDRESSES,
            json!({
            "input":
            {
                "ordering": "ASC",
                "orderingCondition": "UPDATED_AT",
                "pagination": {
                    "pageNum": addresses.len() / page_size,
                    "pageSize": page_size
                }
            }
            }),
        )
        .await
        .unwrap();

    let addresses_response = &response["getManyAddresses"];
    assert_eq!(addresses_response["total"], addresses.len());
    assert_eq!(addresses_response["pageCount"], addresses.len() / page_size);

    let addresses = addresses_response["data"]
        .as_array()
        .expect("Empty response");

    assert_eq!(addresses.len(), page_size);
    check_address(&payload, addresses.last().unwrap())
}
