use anchor_client::solana_sdk::{pubkey::Pubkey, signature::Signer};
use serde_json::json;

mod assert;
mod cmd_utils;
mod common_fixtures;
mod solana;

use solana::setup::Setup;

use common_fixtures::*;
use solana::fixtures::{ADDRESS_ADDR_1, ASSET_ADDR_1};

#[tokio::test(flavor = "multi_thread")]
async fn solana_works() {
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

    t.print("Check that initial authority matches the key of contract deployer");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &authority_pubkey})
    );

    t.print("Assign authority to a new address");
    std::env::set_var("PRIVATE_KEY", &authority_secret);
    assert_tx_output!(t.exec(["authority", "set", &publisher_pubkey]));

    t.print("Make sure that authority has changed");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &publisher_pubkey })
    );

    t.print("Use the private key of the new authority to change the authority back");
    assert_tx_output!(t.exec([
        "authority",
        "set",
        &authority_pubkey,
        "--private-key",
        &authority_secret,
    ]));

    t.print("Make sure that authority has changed back");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &authority_pubkey })
    );

    t.print("Check that initial stake configuration is empty");
    assert_json_output!(
        t.exec(["configuration", "get-stake"]),
        json!({
            "configuration": {
                "token": Pubkey::default().to_string(),
                "unlock_duration": 0,
                "validator_stake": 0.to_string(),
                "tracer_stake": 0.to_string(),
                "publisher_stake": 0.to_string(),
                "authority_stake": 0.to_string()
            }
        })
    );

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

    t.print("Make sure that the new stake configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-stake"]),
        json!({
            "configuration": {
                "token": stake_mint,
                "unlock_duration": unlock_duration,
                "validator_stake": validator_stake.to_string(),
                "tracer_stake": tracer_stake.to_string(),
                "publisher_stake": publisher_stake.to_string(),
                "authority_stake": authority_stake.to_string()
            }
        })
    );

    t.print("Check that initial reward configuration is empty");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({
            "configuration": {
                "token": Pubkey::default().to_string(),
                "address_confirmation_reward": 0.to_string(),
                "address_tracer_reward": 0.to_string(),
                "asset_confirmation_reward": 0.to_string(),
                "asset_tracer_reward": 0.to_string()
            }
        })
    );

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

    t.print("Make sure that the new reward configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({
            "configuration": {
                "token": reward_mint,
                "address_confirmation_reward": address_confirmation_reward.to_string(),
                "address_tracer_reward": address_tracer_reward.to_string(),
                "asset_confirmation_reward": asset_confirmation_reward.to_string(),
                "asset_tracer_reward": asset_tracer_reward.to_string()
            }
        })
    );

    t.print("Make sure that the reporter 1 does not exist yet");
    assert_error_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        "Error: Account not found"
    );

    t.print("Create authority reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        REPORTER_UUID_1,
        &authority_pubkey,
        "authority",
        "HAPI Authority",
        "https://hapi.one/reporter/authority",
    ]));

    t.print("Check that the authority reporter has been created");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        json!({ "reporter": {
            "id": REPORTER_UUID_1,
            "account": authority_pubkey,
            "role": "Authority",
            "name": "HAPI Authority",
            "url": "https://hapi.one/reporter/authority",
            "stake": "0",
            "status": "Inactive",
            "unlock_timestamp": 0
        }})
    );

    t.print("Make sure that reporter counter has increased");
    assert_json_output!(t.exec(["reporter", "count"]), json!({ "count": 1 }));

    t.print("Check authority's token balance");
    let json = assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &authority_pubkey]),
        json!({ "balance": "1000000000" })
    );

    let authority_balance = json["balance"].as_str().unwrap().parse::<u64>().unwrap();

    t.print("Check publisher's token balance");
    let json = assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
        json!({ "balance": "1000000000" })
    );

    let publisher_balance = json["balance"].as_str().unwrap().parse::<u64>().unwrap();

    t.print("Activate authority reporter");
    assert_tx_output!(t.exec(["reporter", "activate"]));

    t.print("Check authority's token balance after activation");
    assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &authority_pubkey,]),
        json!({ "balance": (authority_balance - authority_stake).to_string(), })
    );

    let authority_balance = authority_balance - authority_stake;

    t.print("Send tokens from authority to reporter");
    assert_tx_output!(t.exec([
        "token",
        "transfer",
        &stake_mint,
        &publisher_pubkey,
        &publisher_stake.to_string(),
    ]));

    let publisher_balance = publisher_balance + publisher_stake;
    let authority_balance = authority_balance - publisher_stake;

    t.print("Check publisher's token balance");
    assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
        json!({ "balance": publisher_balance.to_string() })
    );

    t.print("Create publisher reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        REPORTER_UUID_2,
        &publisher_pubkey,
        "publisher",
        "HAPI Publisher",
        "https://hapi.one/reporter/publisher",
    ]));

    t.print("Check that the publisher reporter has been created");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_2]),
        json!({ "reporter": {
            "id": REPORTER_UUID_2,
            "account": &publisher_pubkey,
            "role": "Publisher",
            "name": "HAPI Publisher",
            "url": "https://hapi.one/reporter/publisher",
            "stake": "0",
            "status": "Inactive",
            "unlock_timestamp": 0
        }})
    );

    t.print("Activate publisher reporter");
    assert_tx_output!(t.exec(["reporter", "activate", "--private-key", &publisher_secret]));

    t.print("Check publisher's token balance after activation");
    assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &publisher_pubkey]),
        json!({ "balance": (publisher_balance - publisher_stake).to_string() })
    );

    t.print("Get reporters list");
    sort_assert_json_output!(
        t.exec(["reporter", "list"]),
        json!({ "reporters": [
            {
                "id": REPORTER_UUID_1,
                "account": authority_pubkey,
                "role": "Authority",
                "name": "HAPI Authority",
                "url": "https://hapi.one/reporter/authority",
                "stake": authority_stake.to_string(),
                "status": "Active",
                "unlock_timestamp": 0
            },
            {
                "id": REPORTER_UUID_2,
                "account": publisher_pubkey,
                "role": "Publisher",
                "name": "HAPI Publisher",
                "url": "https://hapi.one/reporter/publisher",
                "stake": publisher_stake.to_string(),
                "status": "Active",
                "unlock_timestamp": 0
            }
        ]}),
        "reporters"
    );

    t.print("Make sure that reporter counter has increased");
    assert_json_output!(t.exec(["reporter", "count"]), json!({ "count": 2 }));

    t.print("Create a case by authority");
    assert_tx_output!(t.exec(["case", "create", CASE_UUID_1, CASE_NAME_1, CASE_URL_1]));

    t.print("Verify that the case has been created");
    assert_json_output!(
        t.exec(["case", "get", CASE_UUID_1]),
        json!({ "case": {
            "id": CASE_UUID_1,
            "name": CASE_NAME_1,
            "url": CASE_URL_1,
            "status": "Open",
            "reporter_id": REPORTER_UUID_1,
        }})
    );

    t.print("Verify the case count has increased");
    assert_json_output!(t.exec(["case", "count"]), json!({ "count": 1 }));

    t.print("Create an address by authority");
    assert_tx_output!(t.exec([
        "address",
        "create",
        ADDRESS_ADDR_1,
        CASE_UUID_1,
        ADDRESS_CATEGORY_1,
        ADDRESS_RISK_1,
    ]));

    t.print("Verify that the address has been created");
    assert_json_output!(
        t.exec(["address", "get", ADDRESS_ADDR_1]),
        json!({ "address": {
            "address": ADDRESS_ADDR_1,
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 5,
            "category": ADDRESS_CATEGORY_1,
            "confirmations": 0,
        }})
    );

    t.print("Verify the address count has increased");
    assert_json_output!(t.exec(["address", "count"]), json!({ "count": 1 }));

    t.print("List addresses");
    assert_json_output!(
        t.exec(["address", "list"]),
        json!({ "addresses": [
            {
                "address": ADDRESS_ADDR_1,
                "case_id": CASE_UUID_1,
                "reporter_id": REPORTER_UUID_1,
                "risk": 5,
                "category": ADDRESS_CATEGORY_1,
                "confirmations": 0,
            }
        ]})
    );

    t.print("Update the address");
    assert_tx_output!(t.exec([
        "address",
        "update",
        ADDRESS_ADDR_1,
        CASE_UUID_1,
        "Scam",
        "6",
    ]));

    t.print("Verify that the address has been updated");
    assert_json_output!(
        t.exec(["address", "get", ADDRESS_ADDR_1]),
        json!({ "address": {
            "address": ADDRESS_ADDR_1,
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 6,
            "category": "Scam",
            "confirmations": 0,
        }})
    );

    t.print("Confirm the address");
    assert_tx_output!(t.exec([
        "address",
        "confirm",
        ADDRESS_ADDR_1,
        "--private-key",
        &publisher_secret
    ]));

    t.print("Verify that the address has been confirmed");
    assert_json_output!(
        t.exec(["address", "get", ADDRESS_ADDR_1]),
        json!({ "address": {
            "address": ADDRESS_ADDR_1,
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 6,
            "category": "Scam",
            "confirmations": 1,
        }})
    );

    t.print("Create an asset by authority");
    assert_tx_output!(t.exec([
        "asset",
        "create",
        ASSET_ADDR_1,
        ASSET_ID_1,
        CASE_UUID_1,
        ASSET_CATEGORY_1,
        ASSET_RISK_1,
    ]));

    t.print("Verify the asset count has increased");
    assert_json_output!(t.exec(["asset", "count"]), json!({ "count": 1 }));

    t.print("List assets");
    assert_json_output!(
        t.exec(["asset", "list"]),
        json!({ "assets": [
            {
                "address": ASSET_ADDR_1,
                "asset_id": ASSET_ID_1,
                "case_id": CASE_UUID_1,
                "reporter_id": REPORTER_UUID_1,
                "risk": 7,
                "category": ASSET_CATEGORY_1,
                "confirmations": 0,
            }
        ]})
    );

    t.print("Update the asset");
    assert_tx_output!(t.exec([
        "asset",
        "update",
        ASSET_ADDR_1,
        ASSET_ID_1,
        CASE_UUID_1,
        "Scam",
        "6",
    ]));

    t.print("Verify that the asset has been updated");
    assert_json_output!(
        t.exec(["asset", "get", ASSET_ADDR_1, ASSET_ID_1]),
        json!({ "asset": {
            "address": ASSET_ADDR_1,
            "asset_id": ASSET_ID_1,
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 6,
            "category": "Scam",
            "confirmations": 0,
        }})
    );

    t.print("Confirm the asset");
    assert_tx_output!(t.exec([
        "asset",
        "confirm",
        ASSET_ADDR_1,
        ASSET_ID_1,
        "--private-key",
        &publisher_secret
    ]));

    t.print("Verify that the asset has been confirmed");
    assert_json_output!(
        t.exec(["asset", "get", ASSET_ADDR_1, ASSET_ID_1]),
        json!({ "asset": {
            "address": ASSET_ADDR_1,
            "asset_id": ASSET_ID_1,
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 6,
            "category": "Scam",
            "confirmations": 1,
        }})
    );

    t.print("Close the case by authority");
    assert_tx_output!(t.exec([
        "case",
        "update",
        CASE_UUID_1,
        "closed case",
        "https://hapi.one/case/closed",
        "closed"
    ]));

    t.print("Verify that the case has been closed");
    assert_json_output!(
        t.exec(["case", "get", CASE_UUID_1]),
        json!({ "case": {
            "id": CASE_UUID_1,
            "name": "closed case",
            "url": "https://hapi.one/case/closed",
            "status": "Closed",
            "reporter_id": REPORTER_UUID_1,
        }})
    );

    t.print("Deactivate authority reporter");
    let unlock_timestamp = {
        let tx_hash = assert_tx_output!(t.exec(["reporter", "deactivate"]));
        let timestamp = t.get_tx_timestamp(&tx_hash).await;
        timestamp + unlock_duration
    };

    t.print("Verify that authority reporter is being deactivated");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        json!({ "reporter": {
            "id": REPORTER_UUID_1,
            "account": authority_pubkey,
            "role": "Authority",
            "name": "HAPI Authority",
            "url": "https://hapi.one/reporter/authority",
            "stake": authority_stake.to_string(),
            "status": "Unstaking",
            "unlock_timestamp": unlock_timestamp
        }})
    );

    tokio::time::sleep(std::time::Duration::from_secs(unlock_duration)).await;

    t.print("Unstake authority reporter");
    assert_tx_output!(t.exec(["reporter", "unstake"]));

    let authority_balance = authority_balance + authority_stake;

    t.print("Verify that the stake has been transferred back to the authority");
    assert_json_output!(
        t.exec(["token", "balance", &stake_mint, &authority_pubkey]),
        json!({ "balance": authority_balance.to_string() })
    );

    t.print("Make sure that the status of the authority reporter is now Inactive");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        json!({ "reporter": {
            "id": REPORTER_UUID_1,
            "account": authority_pubkey,
            "role": "Authority",
            "name": "HAPI Authority",
            "url": "https://hapi.one/reporter/authority",
            "stake": "0",
            "status": "Inactive",
            "unlock_timestamp": 0
        }})
    );

    t.print("Update publisher reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "update",
        REPORTER_UUID_2,
        &publisher_pubkey,
        "publisher",
        "HAPI Publisher+",
        "https://hapi.one/reporter/new_publisher",
    ]));

    t.print("Verify that the publisher reporter has been updated");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_2]),
        json!({ "reporter": {
            "id": REPORTER_UUID_2,
            "account": publisher_pubkey,
            "role": "Publisher",
            "name": "HAPI Publisher+",
            "url": "https://hapi.one/reporter/new_publisher",
            "stake": "12",
            "status": "Active",
            "unlock_timestamp": 0
        }})
    );
}
