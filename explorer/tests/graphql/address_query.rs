use super::replacer;
use crate::helpers::{FromTestPayload, RequestSender, TestApp, TestData};

use {
    hapi_core::client::{entities::address::Address, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_ADDRESS_QUERY: &str = "
    query GetAddress($address: String!, $networkId: String!) {
        getAddress(address: $address, networkId: $networkId) {
            networkId
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
                networkId
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

impl FromTestPayload for TestData<Address> {
    fn from_payload(payload: &PushPayload, network_id: &str) -> TestData<Address> {
        let entity = match &payload.data {
            PushData::Address(address) => address,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network_id: network_id.to_string(),
        }
    }
}

fn check_address(address: &TestData<Address>, value: &Value) {
    assert_eq!(value["networkId"], address.network_id);

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
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
        .await;

    for payload in addresses {
        let response = sender
            .send_graphql(
                GET_ADDRESS_QUERY,
                json!({
                    "address": payload.data.address,
                    "networkId": payload.network_id
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
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_ADDRESSES,
            json!({
            "input":
            {
              "ordering": "ASC",
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
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
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
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
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

#[tokio::test]
async fn get_searched_addresses_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
        .await;

    for payload in addresses {
        let response = sender
            .send_graphql(
                GET_MANY_ADDRESSES,
                json!({
                "input":
                {
                    "search" : &payload.network_id[0..payload.network_id.len() - 1],
                    "ordering": "ASC",
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
