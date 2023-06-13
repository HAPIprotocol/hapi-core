use serde_json::json;

mod setup;
use setup::{CmdOutput, Setup, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2};

mod util;
use util::{is_tx_match, to_json};

#[tokio::test]
async fn it_works() {
    let t = Setup::new();

    // Check that initial authority matches the key of contract deployer
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_1 }));
    }

    // Assign authority to a new address
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "set", PUBLIC_KEY_2])
            .unwrap_or_else(|e| panic!("{}", e));

        assert!(is_tx_match(to_json(&stdout)), "expected tx hash");
    }

    // Make sure that authority has changed
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_2 }));
    }

    // Use the private key of the new authority to change the authority back
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

    // Make sure that authority has changed back
    {
        let CmdOutput { stdout, .. } = t
            .exec(["authority", "get"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(to_json(&stdout), json!({ "authority": PUBLIC_KEY_1 }));
    }

    // Check that initial configuration is empty
    {
        let CmdOutput {
            success, stderr, ..
        } = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert!(!success);
        assert_eq!(stderr, "Error: Ethers error: `get_stake_configuration` reverted with: Stake configuration is not set");
    }

    // Update stake configuration
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

    // Make sure that the new stake configuration is applied
    {
        let CmdOutput { stdout, .. } = t
            .exec(["configuration", "get-stake"])
            .unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(
            to_json(&stdout),
            json!({ "configuration": { "token": t.token, "unlock_duration": 600, "validator_stake": "10", "tracer_stake": "11", "publisher_stake": "12", "authority_stake": "13" }}),
        );
    }
}
