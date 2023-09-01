use anchor_client::solana_sdk::signature::Signer;
use serde_json::json;

mod assert;
mod cmd_utils;
mod solana;

use solana::setup::Setup;
use spl_token::solana_program::pubkey::Pubkey;

use crate::solana::{AUTHORITY_UUID, PUBLISHERY_UUID};

#[tokio::test(flavor = "multi_thread")]
async fn solana_cli_works() {
    println!("Running solana-cli tests");
    let t = Setup::new().await;

    let unlock_duration = 3;
    let validator_stake = 10;
    let tracer_stake = 11;
    let publisher_stake = 12;
    let authority_stake = 13;
    let address_confirmation_reward = 5;
    let address_tracer_reward = 6;
    let asset_confirmation_reward = 7;
    let asset_tracer_reward = 8;

    let authority_pubkey = t.authority.pubkey().to_string();
    let authority_secret = t.authority.to_base58_string();
    let publisher_pubkey = t.publisher.pubkey().to_string();
    let publisher_secret = t.publisher.to_base58_string();
    let stake_mint = t.stake_mint.pubkey().to_string();
    let reward_mint = t.reward_mint.pubkey().to_string();

    // t.print("Check that initial authority matches the key of contract deployer");
    // assert_json_output!(
    //     t.exec(["authority", "get"]),
    //     json!({ "authority": &authority_pubkey})
    // );

    // t.print("Assign authority to a new address");
    // std::env::set_var("PRIVATE_KEY", &authority_secret);
    // assert_tx_output!(t.exec(["authority", "set", &publisher_pubkey]));

    // t.print("Make sure that authority has changed");
    // assert_json_output!(
    //     t.exec(["authority", "get"]),
    //     json!({ "authority": &publisher_pubkey })
    // );

    // t.print("Use the private key of the new authority to change the authority back");
    // assert_tx_output!(t.exec([
    //     "authority",
    //     "set",
    //     &authority_pubkey,
    //     "--private-key",
    //     &authority_secret,
    // ]));

    // t.print("Make sure that authority has changed back");
    // assert_json_output!(
    //     t.exec(["authority", "get"]),
    //     json!({ "authority": &authority_pubkey })
    // );

    // t.print("Check that initial stake configuration is empty");
    // assert_json_output!(
    //     t.exec(["configuration", "get-stake"]),
    //     json!({
    //         "configuration": {
    //             "token": Pubkey::default().to_string(),
    //             "unlock_duration": 0,
    //             "validator_stake": 0.to_string(),
    //             "tracer_stake": 0.to_string(),
    //             "publisher_stake": 0.to_string(),
    //             "authority_stake": 0.to_string()
    //         }
    //     })
    // );

    t.print("Update stake configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-stake",
        &stake_mint,
        &unlock_duration.to_string(),
        &validator_stake.to_string(),
        &tracer_stake.to_string(),
        &publisher_stake.to_string(),
        &authority_stake.to_string(),
    ]));

    // t.print("Make sure that the new stake configuration is applied");
    // assert_json_output!(
    //     t.exec(["configuration", "get-stake"]),
    //     json!({
    //         "configuration": {
    //             "token": stake_mint,
    //             "unlock_duration": unlock_duration,
    //             "validator_stake": validator_stake.to_string(),
    //             "tracer_stake": tracer_stake.to_string(),
    //             "publisher_stake": publisher_stake.to_string(),
    //             "authority_stake": authority_stake.to_string()
    //         }
    //     })
    // );

    // t.print("Check that initial reward configuration is empty");
    // assert_json_output!(
    //     t.exec(["configuration", "get-reward"]),
    //     json!({
    //         "configuration": {
    //             "token": Pubkey::default().to_string(),
    //             "address_confirmation_reward": 0.to_string(),
    //             "address_tracer_reward": 0.to_string(),
    //             "asset_confirmation_reward": 0.to_string(),
    //             "asset_tracer_reward": 0.to_string()
    //         }
    //     })
    // );

    t.print("Update reward configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-reward",
        &reward_mint,
        &address_confirmation_reward.to_string(),
        &address_tracer_reward.to_string(),
        &asset_confirmation_reward.to_string(),
        &asset_tracer_reward.to_string(),
    ]));

    // t.print("Make sure that the new reward configuration is applied");
    // assert_json_output!(
    //     t.exec(["configuration", "get-reward"]),
    //     json!({
    //         "configuration": {
    //             "token": reward_mint,
    //             "address_confirmation_reward": address_confirmation_reward.to_string(),
    //             "address_tracer_reward": address_tracer_reward.to_string(),
    //             "asset_confirmation_reward": asset_confirmation_reward.to_string(),
    //             "asset_tracer_reward": asset_tracer_reward.to_string()
    //         }
    //     })
    // );

    // t.print("Make sure that the reporter 1 does not exist yet");
    // assert_error_output!(
    //     t.exec(["reporter", "get", AUTHORITY_UUID]),
    //     "Error: Anchor Rpc error: Account not found\n\nCaused by:\n    Account not found"
    // );

    t.print("Create authority reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        AUTHORITY_UUID,
        &authority_pubkey,
        "authority",
        "HAPI Authority",
        "https://hapi.one/reporter/authority",
    ]));

    // t.print("Check that the authority reporter has been created");
    // assert_json_output!(
    //     t.exec(["reporter", "get", AUTHORITY_UUID]),
    //     json!({ "reporter": {
    //         "id": AUTHORITY_UUID,
    //         "account": authority_pubkey,
    //         "role": "authority",
    //         "name": "HAPI Authority",
    //         "url": "https://hapi.one/reporter/authority",
    //         "stake": "0",
    //         "status": "inactive",
    //         "unlock_timestamp": 0
    //     }})
    // );

    // t.print("Update authority reporter");
    // assert_tx_output!(t.exec([
    //     "reporter",
    //     "update",
    //     AUTHORITY_UUID,
    //     &authority_pubkey,
    //     "authority",
    //     "Updated HAPI Authority",
    //     "https://hapi.one/reporter/updated_authority",
    // ]));

    // t.print("Check that the authority reporter has been updated");
    // assert_json_output!(
    //     t.exec(["reporter", "get", AUTHORITY_UUID]),
    //     json!({ "reporter": {
    //         "id": AUTHORITY_UUID,
    //         "account": authority_pubkey,
    //         "role": "authority",
    //         "name": "Updated HAPI Authority",
    //         "url": "https://hapi.one/reporter/updated_authority",
    //         "stake": "0",
    //         "status": "inactive",
    //         "unlock_timestamp": 0
    //     }})
    // );

    // t.print("Make sure that reporter counter has increased");
    // assert_json_output!(t.exec(["reporter", "count"]), json!({ "count": 1 }));

    // t.print("Check authority's token balance");
    // let json = assert_json_output!(
    //     t.exec(["token", "balance", &stake_mint, &authority_pubkey]),
    //     json!({ "balance": "1000000000" })
    // );

    // let authority_balance = json["balance"].as_str().unwrap().parse::<u64>().unwrap();

    // t.print("Check publisher's token balance");
    // let json = assert_json_output!(
    //     t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
    //     json!({ "balance": "1000000000" })
    // );

    // let publisher_balance = json["balance"].as_str().unwrap().parse::<u64>().unwrap();

    t.print("Activate authority reporter");
    assert_tx_output!(t.exec(["reporter", "activate"]));

    // t.print("Check authority's token balance after activation");
    // assert_json_output!(
    //     t.exec(["token", "balance", &stake_mint, &authority_pubkey,]),
    //     json!({ "balance": (authority_balance - authority_stake).to_string(), })
    // );

    // let authority_balance = authority_balance - authority_stake;

    // t.print("Send tokens from authority to reporter");
    // assert_tx_output!(t.exec([
    //     "token",
    //     "transfer",
    //     &stake_mint,
    //     &publisher_pubkey,
    //     &publisher_stake.to_string(),
    // ]));

    // let publisher_balance = publisher_balance + publisher_stake;
    // let authority_balance = authority_balance - publisher_stake;

    // t.print("Check publisher's token balance");
    // assert_json_output!(
    //     t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
    //     json!({ "balance": publisher_balance.to_string() })
    // );

    t.print("Create publisher reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        PUBLISHERY_UUID,
        &publisher_pubkey,
        "publisher",
        "HAPI Publisher",
        "https://hapi.one/reporter/publisher",
    ]));

    // t.print("Check that the publisher reporter has been created");
    // assert_json_output!(
    //     t.exec(["reporter", "get", PUBLISHERY_UUID]),
    //     json!({ "reporter": {
    //         "id": PUBLISHERY_UUID,
    //         "account": &publisher_pubkey,
    //         "role": "publisher",
    //         "name": "HAPI Publisher",
    //         "url": "https://hapi.one/reporter/publisher",
    //         "stake": "0",
    //         "status": "inactive",
    //         "unlock_timestamp": 0
    //     }})
    // );

    t.print("Activate publisher reporter");
    assert_tx_output!(t.exec(["reporter", "activate", "--private-key", &publisher_secret]));

    // t.print("Check publisher's token balance after activation");
    // assert_json_output!(
    //     t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
    //     json!({ "balance": (publisher_balance - publisher_stake).to_string() })
    // );

    // t.print("Get reporters list");
    // sort_assert_json_output!(
    //     t.exec(["reporter", "list"]),
    //     json!({ "reporters": [
    //         {
    //             "id": AUTHORITY_UUID,
    //             "account": authority_pubkey,
    //             "role": "authority",
    //             "name": "HAPI Authority",
    //             "url": "https://hapi.one/reporter/authority",
    //             "stake": authority_stake.to_string(),
    //             "status": "active",
    //             "unlock_timestamp": 0
    //         },
    //         {
    //             "id": PUBLISHERY_UUID,
    //             "account": publisher_pubkey,
    //             "role": "publisher",
    //             "name": "HAPI Publisher",
    //             "url": "https://hapi.one/reporter/publisher",
    //             "stake": publisher_stake.to_string(),
    //             "status": "active",
    //             "unlock_timestamp": 0
    //         }
    //     ]}),
    //     "reporters"
    // );

    // t.print("Make sure that reporter counter has increased");
    // assert_json_output!(t.exec(["reporter", "count"]), json!({ "count": 2 }));
}
