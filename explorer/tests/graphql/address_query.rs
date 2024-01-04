use crate::helpers::{RequestSender, TestApp};

use {
    hapi_core::{
        client::{entities::address::Address, events::EventName},
        HapiCoreNetwork,
    },
    hapi_indexer::PushData,
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

fn check_address(payload: &Address, address: &Value, network_id: &HapiCoreNetwork) {
    let replacer = |v: &Value| {
        v.to_string()
            .replace("\"", "")
            .replace("_", "")
            .to_lowercase()
    };

    assert_eq!(
        replacer(&address["network"]),
        network_id.to_string().to_lowercase()
    );
    assert_eq!(address["address"], payload.address);
    assert_eq!(address["caseId"], payload.case_id.to_string());
    assert_eq!(address["reporterId"], payload.reporter_id.to_string());
    assert_eq!(address["risk"], payload.risk);
    assert_eq!(
        replacer(&address["category"]),
        payload.category.to_string().to_lowercase()
    );
    assert_eq!(address["confirmations"], payload.confirmations.to_string());
}

#[tokio::test]
async fn get_address_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities(&sender, EventName::UpdateAddress)
        .await;

    for (payload, network) in addresses {
        let addr_payload = match payload {
            PushData::Address(addr) => addr,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_ADDRESS_QUERY,
                json!({
                    "address": addr_payload.address,
                    "network": network.to_string().to_uppercase()
                }),
            )
            .await;

        let address = &response["getAddress"];
        check_address(&addr_payload, address, &network);
    }
}

#[tokio::test]
async fn get_many_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities(&sender, EventName::UpdateAddress)
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
        .await;

    let addresses_response = &response["getManyAddresses"];
    assert_eq!(addresses_response["total"], addresses.len());

    for (index, address) in addresses_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let (payload, network) = addresses.get(index).expect("Invalid index");
        let addr_payload = match payload {
            PushData::Address(addr) => addr,
            _ => panic!("Invalid type"),
        };

        check_address(addr_payload, address, network)
    }
}

#[tokio::test]
async fn get_filtered_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities(&sender, EventName::UpdateAddress)
        .await;

    for (payload, network) in addresses {
        let addr_payload = match payload {
            PushData::Address(addr) => addr,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_MANY_ADDRESSES,
                json!({
                "input":
                {
                    "filtering": {
                        "network": network.to_string().to_uppercase(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }

                }),
            )
            .await;

        let addresses_response = &response["getManyAddresses"];
        assert_eq!(addresses_response["total"], 1);

        let address = addresses_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_address(&addr_payload, address, network)
    }
}

#[tokio::test]
async fn get_paginated_addresses_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let addresses = test_app
        .setup_entities(&sender, EventName::UpdateAddress)
        .await;

    let (payload, network) = addresses.last().expect("Invalid index");
    let addr_payload = match payload {
        PushData::Address(addr) => addr,
        _ => panic!("Invalid type"),
    };

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
        .await;

    let addresses_response = &response["getManyAddresses"];
    assert_eq!(addresses_response["total"], addresses.len());
    assert_eq!(addresses_response["pageCount"], addresses.len() / page_size);

    let addresses = addresses_response["data"]
        .as_array()
        .expect("Empty response");

    assert_eq!(addresses.len(), page_size);
    check_address(&addr_payload, addresses.last().unwrap(), network)
}
