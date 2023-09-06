use serde_json::json;
use std::{thread::sleep, time::Duration};

mod assert;
mod cmd_utils;
mod common_fixtures;
mod evm;

use common_fixtures::*;
use evm::{
    fixtures::*,
    setup::Setup,
    util::{is_tx_match, to_checksum},
};

#[tokio::test]
async fn evm_works() {
    let t = Setup::new();

    let unlock_duration = 3;
    let validator_stake = 10;
    let tracer_stake = 11;
    let publisher_stake = 12;
    let authority_stake = 13;
    let address_confirmation_reward = 5;
    let tracer_reward = 6;

    t.print("Check that initial authority matches the key of contract deployer");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": PUBLIC_KEY_1 })
    );

    t.print("Assign authority to a new address");
    assert_tx_output!(t.exec(["authority", "set", PUBLIC_KEY_2]));

    t.print("Make sure that authority has changed");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": PUBLIC_KEY_2 })
    );

    t.print("Use the private key of the new authority to change the authority back");
    assert_tx_output!(t.exec([
        "authority",
        "set",
        PUBLIC_KEY_1,
        "--private-key",
        PRIVATE_KEY_2,
    ]));

    t.print("Make sure that authority has changed back");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": PUBLIC_KEY_1 })
    );

    t.print("Check that initial stake configuration is empty");
    assert_error_output!(
        t.exec(["configuration", "get-stake"]),
        "Error: Ethers error: `stake_configuration` reverted with: Stake configuration is not set"
    );

    t.print("Update stake configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-stake",
        &t.token_contract,
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
                "token": t.token_contract,
                "unlock_duration": unlock_duration,
                "validator_stake": validator_stake.to_string(),
                "tracer_stake": tracer_stake.to_string(),
                "publisher_stake": publisher_stake.to_string(),
                "authority_stake": authority_stake.to_string()
            }
        })
    );

    t.print("Check that initial reward configuration is empty");
    assert_error_output!(
        t.exec(["configuration", "get-reward"]),
        "Error: Ethers error: `reward_configuration` reverted with: Reward configuration is not set"
    );

    t.print("Update reward configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-reward",
        &t.token_contract,
        &address_confirmation_reward.to_string(),
        &tracer_reward.to_string(),
    ]));

    t.print("Make sure that the new reward configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({
            "configuration": {
                "token": t.token_contract,
                "address_confirmation_reward": address_confirmation_reward.to_string(),
                "tracer_reward": tracer_reward.to_string()
            }
        })
    );

    t.print("Make sure that the reporter 1 does not exist yet");
    assert_error_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        "Error: Ethers error: `get_reporter` reverted with: Reporter does not exist"
    );

    t.print("Create authority reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        REPORTER_UUID_1,
        PUBLIC_KEY_1,
        "Authority",
        "HAPI Authority",
        "https://hapi.one/reporter/authority",
    ]));

    t.print("Check that the authority reporter has been created");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        json!({ "reporter": {
            "id": REPORTER_UUID_1,
            "account": to_checksum(PUBLIC_KEY_1),
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

    t.print("Try to activate the authority reporter without allowance");
    assert_error_output!(
        t.exec(["reporter", "activate"]),
        "Error: Ethers error: `activate_reporter` reverted with: ERC20: insufficient allowance"
    );

    t.print("Check authority's token balance");
    let json = assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1]),
        json!({ "balance": "1000000000000000000000000" })
    );

    let authority_balance = json["balance"].as_str().unwrap().parse::<u128>().unwrap();

    t.print("Establish token allowance from authority to activate the reporter account");
    assert_tx_output!(t.exec([
        "token",
        "approve",
        &t.token_contract,
        &t.contract_address,
        &authority_stake.to_string(),
    ]));

    t.print("Activate authority reporter");
    assert_tx_output!(t.exec(["reporter", "activate"]));

    t.print("Check authority's token balance after activation");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1]),
        json!({ "balance": (authority_balance - authority_stake).to_string() })
    );

    t.print("Send tokens from authority to reporter");
    assert_tx_output!(t.exec([
        "token",
        "transfer",
        &t.token_contract,
        PUBLIC_KEY_2,
        &publisher_stake.to_string()
    ]));

    let authority_balance = authority_balance - publisher_stake;

    t.print("Check publisher's token balance");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_2]),
        json!({ "balance": publisher_stake.to_string() })
    );

    t.print("Create publisher reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        REPORTER_UUID_2,
        PUBLIC_KEY_2,
        "Publisher",
        "HAPI Publisher",
        "https://hapi.one/reporter/publisher",
    ]));

    t.print("Check that the publisher reporter has been created");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_2]),
        json!({ "reporter": {
            "id": REPORTER_UUID_2,
            "account": to_checksum(PUBLIC_KEY_2),
            "role": "Publisher",
            "name": "HAPI Publisher",
            "url": "https://hapi.one/reporter/publisher",
            "stake": "0",
            "status": "Inactive",
            "unlock_timestamp": 0
        }})
    );

    t.print("Establish token allowance from publisher to activate the reporter account");
    assert_tx_output!(t.exec([
        "token",
        "approve",
        &t.token_contract,
        &t.contract_address,
        &publisher_stake.to_string(),
        "--private-key",
        PRIVATE_KEY_2,
    ]));

    t.print("Activate publisher reporter");
    assert_tx_output!(t.exec(["reporter", "activate", "--private-key", PRIVATE_KEY_2]));

    t.print("Check publisher's token balance after activation");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_2]),
        json!({ "balance": "0" })
    );

    t.print("Get reporters list");
    assert_json_output!(
        t.exec(["reporter", "list"]),
        json!({ "reporters": [
            {
                "id": REPORTER_UUID_1,
                "account": to_checksum(PUBLIC_KEY_1),
                "role": "Authority",
                "name": "HAPI Authority",
                "url": "https://hapi.one/reporter/authority",
                "stake": authority_stake.to_string(),
                "status": "Active",
                "unlock_timestamp": 0
            },
            {
                "id": REPORTER_UUID_2,
                "account": to_checksum(PUBLIC_KEY_2),
                "role": "Publisher",
                "name": "HAPI Publisher",
                "url": "https://hapi.one/reporter/publisher",
                "stake": publisher_stake.to_string(),
                "status": "Active",
                "unlock_timestamp": 0
            }
        ]})
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
            "address": to_checksum(ADDRESS_ADDR_1),
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 5,
            "category": "Ransomware",
        }})
    );

    t.print("Verify the address count has increased");
    assert_json_output!(t.exec(["address", "count"]), json!({ "count": 1 }));

    t.print("List addresses");
    assert_json_output!(
        t.exec(["address", "list"]),
        json!({ "addresses": [
            {
                "address": to_checksum(ADDRESS_ADDR_1),
                "case_id": CASE_UUID_1,
                "reporter_id": REPORTER_UUID_1,
                "risk": 5,
                "category": "Ransomware",
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
            "address": to_checksum(ADDRESS_ADDR_1),
            "case_id": CASE_UUID_1,
            "reporter_id": REPORTER_UUID_1,
            "risk": 6,
            "category": "Scam",
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
                "address": to_checksum(ASSET_ADDR_1),
                "asset_id": ASSET_ID_1,
                "case_id": CASE_UUID_1,
                "reporter_id": REPORTER_UUID_1,
                "risk": 7,
                "category": "Counterfeit",
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

    t.print("Close the case by authority");
    assert_tx_output!(t.exec([
        "case",
        "update",
        CASE_UUID_1,
        "closed case",
        "https://hapi.one/case/closed",
        "Closed"
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
            "account": to_checksum(PUBLIC_KEY_1),
            "role": "Authority",
            "name": "HAPI Authority",
            "url": "https://hapi.one/reporter/authority",
            "stake": authority_stake.to_string(),
            "status": "Unstaking",
            "unlock_timestamp": unlock_timestamp
        }})
    );

    sleep(Duration::from_secs(unlock_duration));

    t.print("Unstake authority reporter");
    assert_tx_output!(t.exec(["reporter", "unstake"]));

    t.print("Verify that the stake has been transferred back to the authority");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1]),
        json!({ "balance": authority_balance.to_string() })
    );

    t.print("Make sure that the status of the authority reporter is now Inactive");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_1]),
        json!({ "reporter": {
            "id": REPORTER_UUID_1,
            "account": to_checksum(PUBLIC_KEY_1),
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
        PUBLIC_KEY_2,
        "Publisher",
        "HAPI Publisher+",
        "https://hapi.one/reporter/new_publisher",
    ]));

    t.print("Verify that the publisher reporter has been updated");
    assert_json_output!(
        t.exec(["reporter", "get", REPORTER_UUID_2]),
        json!({ "reporter": {
            "id": REPORTER_UUID_2,
            "account": to_checksum(PUBLIC_KEY_2),
            "role": "Publisher",
            "name": "HAPI Publisher+",
            "url": "https://hapi.one/reporter/new_publisher",
            "stake": "12",
            "status": "Active",
            "unlock_timestamp": 0
        }})
    );
}
