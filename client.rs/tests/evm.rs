use serde_json::json;

mod setup;
use setup::{Setup, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2, REPORTER_UUID_1, REPORTER_UUID_2};

mod util;
use util::{is_tx_match, to_checksum, to_json};

#[tokio::test]
async fn evm_works() {
    let t = Setup::new();

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
        "Error: Ethers error: `get_stake_configuration` reverted with: Stake configuration is not set"
    );

    t.print("Update stake configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-stake",
        &t.token_contract,
        "600",
        "10",
        "11",
        "12",
        "13",
    ]));

    t.print("Make sure that the new stake configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-stake"]),
        json!({ "configuration": { "token": t.token_contract, "unlock_duration": 600, "validator_stake": "10", "tracer_stake": "11", "publisher_stake": "12", "authority_stake": "13" }})
    );

    t.print("Check that initial reward configuration is empty");
    assert_error_output!(
            t.exec(["configuration", "get-reward"]),
            "Error: Ethers error: `get_reward_configuration` reverted with: Reward configuration is not set"
        );

    t.print("Update reward configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-reward",
        &t.token_contract,
        "5",
        "6",
    ]));

    t.print("Make sure that the new reward configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({ "configuration": { "token": t.token_contract, "address_confirmation_reward": "5", "tracer_reward": "6" }})
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
        "authority",
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
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1]),
        json!({ "balance": "1000000000" })
    );

    t.print("Establish token allowance from authority to activate the reporter account");
    assert_tx_output!(t.exec([
        "token",
        "approve",
        &t.token_contract,
        &t.contract_address,
        "13",
    ]));

    t.print("Activate authority reporter");
    assert_tx_output!(t.exec(["reporter", "activate"]));

    t.print("Check authority's token balance after activation");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1]),
        json!({ "balance": "999999987" })
    );

    t.print("Send tokens from authority to reporter");
    assert_tx_output!(t.exec(["token", "transfer", &t.token_contract, PUBLIC_KEY_2, "12"]));

    t.print("Check publisher's token balance");
    assert_json_output!(
        t.exec(["token", "balance", &t.token_contract, PUBLIC_KEY_2]),
        json!({ "balance": "12" })
    );

    t.print("Create publisher reporter");
    assert_tx_output!(t.exec([
        "reporter",
        "create",
        REPORTER_UUID_2,
        PUBLIC_KEY_2,
        "publisher",
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
        "12",
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
                "stake": "13",
                "status": "Active",
                "unlock_timestamp": 0
            },
            {
                "id": REPORTER_UUID_2,
                "account": to_checksum(PUBLIC_KEY_2),
                "role": "Publisher",
                "name": "HAPI Publisher",
                "url": "https://hapi.one/reporter/publisher",
                "stake": "12",
                "status": "Active",
                "unlock_timestamp": 0
            }
        ]})
    );
}
