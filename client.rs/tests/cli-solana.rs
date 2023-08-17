use serde_json::json;
use std::{thread::sleep, time::Duration};

use anchor_client::solana_sdk::signature::Signer;

mod assert;
mod cmd_utils;
mod solana;

use solana::{fixtures::*, setup::Setup};

#[tokio::test(flavor = "multi_thread")]
async fn solana_cli_works() {
    println!("Running solana-cli tests");
    let t = Setup::new().await;

    let admin = t.data.get_wallet("admin").keypair.pubkey().to_string();

    t.print("Check that initial authority matches the key of contract deployer");
    assert_json_output!(t.exec(["authority", "get"]), json!({ "authority": &admin}));

    // t.print("Assign authority to a new address");
    // assert_tx_output!(t.exec(["authority", "set", PUBLIC_KEY_2]));

    // t.print("Make sure that authority has changed");
    // assert_json_output!(
    //     t.exec(["authority", "get"]),
    //     json!({ "authority": PUBLIC_KEY_2 })
    // );

    // t.print("Use the private key of the new authority to change the authority back");
    // assert_tx_output!(t.exec([
    //     "authority",
    //     "set",
    //     PUBLIC_KEY_1,
    //     "--private-key",
    //     PRIVATE_KEY_2,
    // ]));

    // t.print("Make sure that authority has changed back");
    // assert_json_output!(
    //     t.exec(["authority", "get"]),
    //     json!({ "authority": PUBLIC_KEY_1 })
    // );
}
