use hapi_core::HapiCoreNetwork;
use hapi_indexer::{configuration::IndexerConfiguration, Indexer};
use std::time::Duration;
use tokio::{spawn, time::sleep, try_join};

#[tokio::test(flavor = "multi_thread")]
async fn indexer_test() {
    let cfg = IndexerConfiguration {
        network: HapiCoreNetwork::Solana,
        rpc_node_url: String::from("Rpc mock url"),
        webhook_url: String::from("Webhook mock url"),
        contract_address: String::from("Default contract address"),
        wait_interval_ms: Duration::from_millis(100),
        state_file: String::from("Default state file"),
    };

    let mut indexer = Indexer::new(cfg).unwrap();

    let indexer_task = spawn(async move { indexer.run().await });
    // TODO: bind sleeping timestamp with rpc mock!!!
    let timer_task = spawn(async move { sleep(Duration::from_millis(1000)).await });

    let _first_run = try_join!(indexer_task, timer_task).expect("Indexer failed");
    // check data
    // check persisted state

    // let _second_run = try_join!(indexer_task, timer_task).expect("Indexer failed");
    // // check data
    // // check persisted state
}
