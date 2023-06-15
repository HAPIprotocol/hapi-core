use serde_json::json;

mod setup;
use setup::{
    CmdOutput, Setup, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2, REPORTER_UUID_1, REPORTER_UUID_2,
};

mod util;
use util::{is_tx_match, to_checksum, to_json};

#[tokio::test]
async fn it_works() {
    let t = Setup::new();

    t.print("Check that initial authority matches the key of contract deployer");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_1 }));
    }

    t.print("Assign authority to a new address");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "set", PUBLIC_KEY_2])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    t.print("Make sure that authority has changed");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_2 }));
    }

    t.print("Use the private key of the new authority to change the authority back");
    {
        let CmdOutput { stdout, .. } = t
            .exec([
                "authority",
                "set",
                PUBLIC_KEY_1,
                "--private-key",
                PRIVATE_KEY_2,
            ])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    t.print("Make sure that authority has changed back");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_1 }));
    }

    t.print("Check that initial stake configuration is empty");
    {
        let CmdOutput {
            success, stderr, ..
        } = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!success);
        assert_eq!(stderr, "Error: Ethers error: `get_stake_configuration` reverted with: Stake configuration is not set");
    }

    t.print("Update stake configuration");
    {
        let CmdOutput { stdout, .. } = t
            .exec([
                "configuration",
                "update-stake",
                &t.token,
                "600",
                "10",
                "11",
                "12",
                "13",
            ])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    t.print("Make sure that the new stake configuration is applied");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&stdout),
            json!({ "configuration": { "token": t.token, "unlock_duration": 600, "validator_stake": "10", "tracer_stake": "11", "publisher_stake": "12", "authority_stake": "13" }}),
        );
    }

    t.print("Check that initial reward configuration is empty");
    {
        let CmdOutput {
            success, stderr, ..
        } = t
            .exec(["configuration", "get-reward"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!success);
        assert_eq!(stderr, "Error: Ethers error: `get_reward_configuration` reverted with: Reward configuration is not set");
    }

    t.print("Update reward configuration");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["configuration", "update-reward", &t.token, "5", "6"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    t.print("Make sure that the new reward configuration is applied");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["configuration", "get-reward"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&stdout),
            json!({ "configuration": { "token": t.token, "address_confirmation_reward": "5", "tracer_reward": "6" }}),
        );
    }

    t.print("Make sure that the reporter 1 does not exist yet");
    {
        let CmdOutput {
            success, stderr, ..
        } = t
            .exec(["reporter", "get", REPORTER_UUID_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!success);
        assert_eq!(
            stderr,
            "Error: Ethers error: `get_reporter` reverted with: Reporter does not exist"
        );
    }

    t.print("Create reporter 1");
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

        assert!(is_tx_match(to_json(&output.stdout)), "expected tx hash");
    }

    t.print("Check that the reporter 1 has been created");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["reporter", "get", REPORTER_UUID_1])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&stdout),
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

    t.print("Create reporter 2");
    {
        let CmdOutput { stdout, .. } = t
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

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    t.print("Check that the reporter 2 has been created");
    {
        let CmdOutput { stdout, .. } = t
            .exec(["reporter", "get", REPORTER_UUID_2])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&stdout),
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
