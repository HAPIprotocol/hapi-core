use super::replacer;
use crate::helpers::{RequestSender, TestApp};

use {
    hapi_core::HapiCoreNetwork,
    hapi_explorer::entity::network::Model as NetworkModel,
    serde_json::{json, Value},
};

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

fn check_network(model: &NetworkModel, value: &Value, backend: &HapiCoreNetwork) {
    assert_eq!(replacer(&value["id"]), model.id.to_string().to_lowercase());
    assert_eq!(value["name"], model.name);
    assert_eq!(
        replacer(&value["backend"]),
        backend.to_string().to_lowercase()
    );
    assert_eq!(value["chainId"], *model.chain_id.as_ref().unwrap());
    assert_eq!(value["authority"], model.authority);
    assert_eq!(value["stakeToken"], model.stake_token);
}

#[tokio::test]
async fn get_network_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());

    for (backend, model) in test_app.networks {
        let response = sender
            .send_graphql(
                GET_NETWORK_QUERY,
                json!({
                    "id": model.id,
                }),
            )
            .await
            .unwrap();

        let network = &response["getNetwork"];
        check_network(&model, network, &backend);
    }
}

#[tokio::test]
async fn get_many_networks_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = test_app.networks;

    let response = sender
        .send_graphql(
            GET_MANY_NETWORKS,
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

    let networks_response = &response["getManyNetworks"];
    assert_eq!(networks_response["total"], networks.len());

    println!("{:?}", networks_response);

    for (index, network) in networks_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let (backend, model) = networks.get(index).expect("Invalid index");

        check_network(model, network, backend)
    }
}

#[tokio::test]
async fn get_filtered_networks_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = test_app.networks;

    for (backend, model) in networks {
        let response = sender
            .send_graphql(
                GET_MANY_NETWORKS,
                json!({
                "input":
                {
                    "filtering": {
                        "backend": backend.to_string().to_uppercase(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
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

        check_network(&model, network, &backend)
    }
}

#[tokio::test]
async fn get_paginated_networks_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let networks = test_app.networks;

    let (backend, model) = networks.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_NETWORKS,
            json!({
            "input":
            {
                "ordering": "ASC",
                "orderingCondition": "UPDATED_AT",
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
    check_network(&model, networks.last().unwrap(), backend)
}
