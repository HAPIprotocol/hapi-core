use serde_json::json;

mod setup;
use setup::{Setup, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2, REPORTER_UUID_1, REPORTER_UUID_2};

mod util;
use util::{is_tx_match, to_checksum, to_json};

#[tokio::test]
async fn it_works() {
    let t = Setup::new();

    t.print("Check that initial authority matches the key of contract deployer");
    {
        let output = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "authority": PUBLIC_KEY_1 })
        );
    }

    t.print("Assign authority to a new address");
    {
        let output = t
            .exec(["authority", "set", PUBLIC_KEY_2])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Make sure that authority has changed");
    {
        let output = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "authority": PUBLIC_KEY_2 })
        );
    }

    t.print("Use the private key of the new authority to change the authority back");
    {
        let output = t
            .exec([
                "authority",
                "set",
                PUBLIC_KEY_1,
                "--private-key",
                PRIVATE_KEY_2,
            ])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Make sure that authority has changed back");
    {
        let output = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "authority": PUBLIC_KEY_1 })
        );
    }

    t.print("Check that initial stake configuration is empty");
    {
        let output = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!output.success);
        assert_eq!(output.stderr, "Error: Ethers error: `get_stake_configuration` reverted with: Stake configuration is not set");
    }

    t.print("Update stake configuration");
    {
        let output = t
            .exec([
                "configuration",
                "update-stake",
                &t.token_contract,
                "600",
                "10",
                "11",
                "12",
                "13",
            ])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Make sure that the new stake configuration is applied");
    {
        let output = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "configuration": { "token": t.token_contract, "unlock_duration": 600, "validator_stake": "10", "tracer_stake": "11", "publisher_stake": "12", "authority_stake": "13" }}),
        );
    }

    t.print("Check that initial reward configuration is empty");
    {
        let output = t
            .exec(["configuration", "get-reward"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!output.success);
        assert_eq!(output.stderr, "Error: Ethers error: `get_reward_configuration` reverted with: Reward configuration is not set");
    }

    t.print("Update reward configuration");
    {
        let output = t
            .exec([
                "configuration",
                "update-reward",
                &t.token_contract,
                "5",
                "6",
            ])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Make sure that the new reward configuration is applied");
    {
        let output = t
            .exec(["configuration", "get-reward"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "configuration": { "token": t.token_contract, "address_confirmation_reward": "5", "tracer_reward": "6" }}),
        );
    }

    t.print("Make sure that the reporter 1 does not exist yet");
    {
        let output = t
            .exec(["reporter", "get", REPORTER_UUID_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!output.success);
        assert_eq!(
            output.stderr,
            "Error: Ethers error: `get_reporter` reverted with: Reporter does not exist"
        );
    }

    t.print("Create authority reporter");
    {
        let output = t
            .exec([
                "reporter",
                "create",
                REPORTER_UUID_1,
                PUBLIC_KEY_1,
                "authority",
                "HAPI Authority",
                "https://hapi.one/reporter/authority",
            ])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Check that the authority reporter has been created");
    {
        let output = t
            .exec(["reporter", "get", REPORTER_UUID_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&output.stdout),
            json!({ "reporter": {
                "id": REPORTER_UUID_1,
                "account": to_checksum(PUBLIC_KEY_1),
                "role": "Authority",
                "name": "HAPI Authority",
                "url": "https://hapi.one/reporter/authority",
                "stake": "0",
                "status": "Inactive",
                "unlock_timestamp": 0
            }}),
        );
    }

    t.print("Make sure that reporter counter has increased");
    {
        let output = t
            .exec(["reporter", "count"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(to_json(&output.stdout), json!({ "count": 1 }));
    }

    t.print("Try to activate the authority reporter without allowance");
    {
        let output = t
            .exec(["reporter", "activate"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!output.success);
        assert_eq!(
            output.stderr,
            "Error: Ethers error: `activate_reporter` reverted with: ERC20: insufficient allowance"
        );
    }

    t.print("Check authority's token balance");
    {
        let output = t
            .exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(to_json(&output.stdout), json!({ "balance": "1000000000" }));
    }

    t.print("Establish token allowance from authority to activate the reporter account");
    {
        let output = t
            .exec([
                "token",
                "approve",
                &t.token_contract,
                &t.contract_address,
                "13",
            ])
            .unwrap_or_else(|e| panic!("{e}"));
        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Activate authority reporter");
    {
        let output = t
            .exec(["reporter", "activate"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Check authority's token balance after activation");
    {
        let output = t
            .exec(["token", "balance", &t.token_contract, PUBLIC_KEY_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);

        assert_eq!(to_json(&output.stdout), json!({ "balance": "999999987" }));
    }

    t.print("Create publisher reporter");
    {
        let output = t
            .exec([
                "reporter",
                "create",
                REPORTER_UUID_2,
                PUBLIC_KEY_2,
                "publisher",
                "HAPI Publisher",
                "https://hapi.one/reporter/publisher",
            ])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Check that the publisher reporter has been created");
    {
        let output = t
            .exec(["reporter", "get", REPORTER_UUID_2])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(output.success);
        assert_eq!(
            to_json(&output.stdout),
            json!({ "reporter": {
                "id": REPORTER_UUID_2,
                "account": to_checksum(PUBLIC_KEY_2),
                "role": "Publisher",
                "name": "HAPI Publisher",
                "url": "https://hapi.one/reporter/publisher",
                "stake": "0",
                "status": "Inactive",
                "unlock_timestamp": 0
            }}),
        );
    }
}
