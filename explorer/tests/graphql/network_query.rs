use super::replacer;
use crate::helpers::{RequestSender, TestApp, TestNetwork};

use serde_json::{json, Value};

const GET_NETWORK_QUERY: &str = "
    query GetNetwork($id: String!) {
        getNetwork(id: $id) {
            id
            name
            backend
            chainId
            authority
            stakeToken
            createdAt
            updatedAt
        }
    }
";
const GET_MANY_NETWORKS: &str = "
    query GetManyNetworks(
        $input: NetworkInput!
    ) {
        getManyNetworks(
            input: $input
        ) {
            data {
                id
                name
                backend
                chainId
                authority
                stakeToken
                createdAt
                updatedAt
            }
            total
            pageCount
        }
    }
";

fn check_network(network: &TestNetwork, value: &Value) {
    let model = &network.model;
    assert_eq!(replacer(&value["id"]), model.id.to_string().to_lowercase());
    assert_eq!(value["name"], model.name);
    assert_eq!(
        replacer(&value["backend"]),
        network.model.backend.to_string().to_lowercase()
    );
    assert_eq!(value["chainId"], *model.chain_id.as_ref().unwrap());
    assert_eq!(value["authority"], model.authority);
    assert_eq!(value["stakeToken"], model.stake_token);
}

#[tokio::test]
async fn get_network_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());

    for data in &test_app.networks {
        let response = sender
            .send_graphql(
                GET_NETWORK_QUERY,
                json!({
                    "id": data.model.id,
                }),
            )
            .await
            .unwrap();

        let network = &response["getNetwork"];
        check_network(&data, network);
    }
}

#[tokio::test]
async fn get_many_networks_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = &test_app.networks;

    let response = sender
        .send_graphql(
            GET_MANY_NETWORKS,
            json!({
            "input":
            {
              "ordering": "ASC",

            }

            }),
        )
        .await
        .unwrap();

    let networks_response = &response["getManyNetworks"];
    assert_eq!(networks_response["total"], networks.len());

    for (index, network) in networks_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let data = networks.get(index).expect("Invalid index");

        check_network(data, network)
    }
}

#[tokio::test]
async fn get_filtered_networks_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = &test_app.networks;

    for data in networks {
        let response = sender
            .send_graphql(
                GET_MANY_NETWORKS,
                json!({
                "input":
                {
                    "filtering": {
                        "name" : data.model.name.clone(),
                    },
                    "ordering": "ASC",
                }

                }),
            )
            .await
            .unwrap();

        let networks_response = &response["getManyNetworks"];
        assert_eq!(networks_response["total"], 1);

        let network = networks_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_network(&data, network)
    }
}

#[tokio::test]
async fn get_paginated_networks_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = &test_app.networks;

    let data = networks.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_NETWORKS,
            json!({
            "input":
            {
                "ordering": "ASC",
                "pagination": {
                    "pageNum": networks.len() / page_size,
                    "pageSize": page_size
                }
            }
            }),
        )
        .await
        .unwrap();

    let networks_response = &response["getManyNetworks"];
    assert_eq!(networks_response["total"], networks.len());
    assert_eq!(networks_response["pageCount"], networks.len() / page_size);

    let networks = networks_response["data"]
        .as_array()
        .expect("Empty response");

    assert_eq!(networks.len(), page_size);
    check_network(&data, networks.last().unwrap())
}

#[tokio::test]
async fn get_searched_networks_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = &test_app.networks;

    for data in networks {
        let chain_id = data.model.chain_id.clone().unwrap();

        let response = sender
            .send_graphql(
                GET_MANY_NETWORKS,
                json!({
                "input":
                {
                    "search" : &chain_id[1..chain_id.len() - 1],
                    "ordering": "ASC",
                }

                }),
            )
            .await
            .unwrap();

        let networks_response = &response["getManyNetworks"];
        assert_eq!(networks_response["total"], 1);

        let network = networks_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_network(&data, network)
    }
}
